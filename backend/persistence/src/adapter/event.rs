use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use rust_decimal::Decimal;
use std::str::FromStr;
use yams_core::models::{Event, Seminar};
use yams_core::ports::{EventRepository, SeminarRepository};
use crate::adapter::SqliteAdapter;
use libsql::params;

#[async_trait]
impl EventRepository for SqliteAdapter {
    async fn find_all(&self) -> Result<Vec<Event>, yams_core::Error> {
        let mut rows = self.db.query("SELECT id, date, location_id, location_name, max_participants, seminar_id FROM events", ())
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let mut events = Vec::new();
        while let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            events.push(map_row_to_event(&row)?);
        }
        Ok(events)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Event>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT id, date, location_id, location_name, max_participants, seminar_id FROM events WHERE id = ?1", 
            params![id.to_string()]
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            Ok(Some(map_row_to_event(&row)?))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, event: Event) -> Result<Event, yams_core::Error> {
        let id = event.id.unwrap_or_else(Uuid::new_v4);
        
        self.db.execute(
            "INSERT INTO events (id, date, location_id, location_name, max_participants, seminar_id) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET 
                date = excluded.date,
                location_id = excluded.location_id,
                location_name = excluded.location_name,
                max_participants = excluded.max_participants,
                seminar_id = excluded.seminar_id",
            params![
                id.to_string(),
                event.date.to_rfc3339(),
                event.location_id.to_string(),
                event.location_name.clone(),
                event.max_participants,
                event.seminar_id.to_string(),
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        let mut saved = event;
        saved.id = Some(id);
        Ok(saved)
    }

    async fn delete(&self, id: Uuid) -> Result<(), yams_core::Error> {
        self.db.execute("DELETE FROM events WHERE id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl SeminarRepository for SqliteAdapter {
    async fn find_all(&self) -> Result<Vec<Seminar>, yams_core::Error> {
        let mut rows = self.db.query("SELECT id, title, price, duration FROM seminars", ())
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let mut seminars = Vec::new();
        while let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            seminars.push(map_row_to_seminar(&row)?);
        }
        Ok(seminars)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Seminar>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT id, title, price, duration FROM seminars WHERE id = ?1", 
            params![id.to_string()]
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            Ok(Some(map_row_to_seminar(&row)?))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, seminar: Seminar) -> Result<Seminar, yams_core::Error> {
        let id = seminar.id.unwrap_or_else(Uuid::new_v4);
        
        self.db.execute(
            "INSERT INTO seminars (id, title, price, duration) 
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET 
                title = excluded.title,
                price = excluded.price,
                duration = excluded.duration",
            params![
                id.to_string(),
                seminar.title.clone(),
                seminar.price.to_string(),
                seminar.duration.map(|d| d.num_milliseconds().to_string()),
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        let mut saved = seminar;
        saved.id = Some(id);
        Ok(saved)
    }

    async fn delete(&self, id: Uuid) -> Result<(), yams_core::Error> {
        // Check if any events use this seminar
        let mut rows = self.db.query("SELECT count(*) FROM events WHERE seminar_id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let count: i64 = rows.next().await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?
            .ok_or_else(|| yams_core::Error::Database("Failed to get count".to_string()))?
            .get(0)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if count > 0 {
            return Err(yams_core::Error::Validation("Cannot delete seminar used by events".to_string()));
        }

        self.db.execute("DELETE FROM seminars WHERE id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        Ok(())
    }
}

fn map_row_to_event(row: &libsql::Row) -> Result<Event, yams_core::Error> {
    let date_str: String = row.get(1).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    let location_id_str: String = row.get(2).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    let seminar_id_str: String = row.get(5).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    
    Ok(Event {
        id: Some(Uuid::parse_str(&row.get::<String>(0).map_err(|e| yams_core::Error::Database(e.to_string()))?)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?),
        date: DateTime::parse_from_rfc3339(&date_str)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?
            .with_timezone(&Utc),
        location_id: Uuid::parse_str(&location_id_str).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        location_name: row.get(3).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        max_participants: row.get(4).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        seminar_id: Uuid::parse_str(&seminar_id_str).map_err(|e| yams_core::Error::Database(e.to_string()))?,
    })
}

fn map_row_to_seminar(row: &libsql::Row) -> Result<Seminar, yams_core::Error> {
    let price_str: String = row.get(2).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    let duration_ms_str: Option<String> = row.get(3).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    
    Ok(Seminar {
        id: Some(Uuid::parse_str(&row.get::<String>(0).map_err(|e| yams_core::Error::Database(e.to_string()))?)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?),
        title: row.get(1).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        price: Decimal::from_str(&price_str).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        duration: duration_ms_str.and_then(|ms| ms.parse::<i64>().ok()).map(Duration::milliseconds),
    })
}
