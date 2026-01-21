use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use yams_core::models::Client;
use yams_core::ports::ClientRepository;
use crate::adapter::SqliteAdapter;
use libsql::params;

#[async_trait]
impl ClientRepository for SqliteAdapter {
    async fn find_all(&self) -> Result<Vec<Client>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT id, first_name, last_name, birthdate, email, mobile_number, customer_number, address_id, consent FROM clients", 
            ()
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let mut clients = Vec::new();
        while let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            let mut client = map_row_to_client(&row)?;
            client.animal_ids = self.get_client_animals(client.id.unwrap()).await?;
            clients.push(client);
        }
        Ok(clients)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Client>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT id, first_name, last_name, birthdate, email, mobile_number, customer_number, address_id, consent FROM clients WHERE id = ?1", 
            params![id.to_string()]
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            let mut client = map_row_to_client(&row)?;
            client.animal_ids = self.get_client_animals(client.id.unwrap()).await?;
            Ok(Some(client))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, client: Client) -> Result<Client, yams_core::Error> {
        let id = client.id.unwrap_or_else(Uuid::new_v4);
        
        self.db.execute(
            "INSERT INTO clients (id, first_name, last_name, birthdate, email, mobile_number, customer_number, address_id, consent) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
             ON CONFLICT(id) DO UPDATE SET 
                first_name = excluded.first_name,
                last_name = excluded.last_name,
                birthdate = excluded.birthdate,
                email = excluded.email,
                mobile_number = excluded.mobile_number,
                customer_number = excluded.customer_number,
                address_id = excluded.address_id,
                consent = excluded.consent",
            params![
                id.to_string(),
                client.first_name.clone(),
                client.last_name.clone(),
                client.birthdate.to_rfc3339(),
                client.email.clone(),
                client.mobile_number.clone(),
                client.customer_number,
                client.address_id.to_string(),
                client.consent,
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        // Update animal relationships
        self.db.execute("DELETE FROM client_animals WHERE client_id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        for animal_id in &client.animal_ids {
            self.db.execute(
                "INSERT INTO client_animals (client_id, animal_id) VALUES (?1, ?2)",
                params![id.to_string(), animal_id.to_string()]
            ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;
        }

        let mut saved = client;
        saved.id = Some(id);
        Ok(saved)
    }

    async fn delete(&self, id: Uuid) -> Result<(), yams_core::Error> {
        self.db.execute("DELETE FROM client_animals WHERE client_id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        self.db.execute("DELETE FROM clients WHERE id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        Ok(())
    }
}

impl SqliteAdapter {
    async fn get_client_animals(&self, client_id: Uuid) -> Result<Vec<Uuid>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT animal_id FROM client_animals WHERE client_id = ?1",
            params![client_id.to_string()]
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let mut animal_ids = Vec::new();
        while let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            let id_str: String = row.get(0).map_err(|e| yams_core::Error::Database(e.to_string()))?;
            animal_ids.push(Uuid::parse_str(&id_str).map_err(|e| yams_core::Error::Database(e.to_string()))?);
        }
        Ok(animal_ids)
    }
}

fn map_row_to_client(row: &libsql::Row) -> Result<Client, yams_core::Error> {
    let birthdate_str: String = row.get(3).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    let address_id_str: String = row.get(7).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    
    Ok(Client {
        id: Some(Uuid::parse_str(&row.get::<String>(0).map_err(|e| yams_core::Error::Database(e.to_string()))?)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?),
        first_name: row.get(1).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        last_name: row.get(2).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        birthdate: DateTime::parse_from_rfc3339(&birthdate_str)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?
            .with_timezone(&Utc),
        email: row.get(4).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        mobile_number: row.get(5).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        customer_number: row.get(6).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        address_id: Uuid::parse_str(&address_id_str).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        consent: row.get(8).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        animal_ids: Vec::new(), // Populated by caller
    })
}
