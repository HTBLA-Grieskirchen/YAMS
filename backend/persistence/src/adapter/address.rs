use async_trait::async_trait;
use uuid::Uuid;
use yams_core::models::Address;
use yams_core::ports::AddressRepository;
use crate::adapter::SqliteAdapter;
use libsql::params;

#[async_trait]
impl AddressRepository for SqliteAdapter {
    async fn find_all(&self) -> Result<Vec<Address>, yams_core::Error> {
        let mut rows = self.db.query("SELECT id, country, postal_code, city, street, street_number, extra FROM addresses", ())
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let mut addresses = Vec::new();
        while let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            addresses.push(map_row_to_address(&row)?);
        }
        Ok(addresses)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Address>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT id, country, postal_code, city, street, street_number, extra FROM addresses WHERE id = ?1", 
            params![id.to_string()]
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            Ok(Some(map_row_to_address(&row)?))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, address: Address) -> Result<Address, yams_core::Error> {
        let id = address.id.unwrap_or_else(Uuid::new_v4);
        
        self.db.execute(
            "INSERT INTO addresses (id, country, postal_code, city, street, street_number, extra) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET 
                country = excluded.country,
                postal_code = excluded.postal_code,
                city = excluded.city,
                street = excluded.street,
                street_number = excluded.street_number,
                extra = excluded.extra",
            params![
                id.to_string(),
                address.country.clone(),
                address.postal_code.clone(),
                address.city.clone(),
                address.street.clone(),
                address.street_number.clone(),
                address.extra.clone(),
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        let mut saved = address;
        saved.id = Some(id);
        Ok(saved)
    }

    async fn delete(&self, id: Uuid) -> Result<(), yams_core::Error> {
        self.db.execute("DELETE FROM addresses WHERE id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        Ok(())
    }
}

fn map_row_to_address(row: &libsql::Row) -> Result<Address, yams_core::Error> {
    Ok(Address {
        id: Some(Uuid::parse_str(&row.get::<String>(0).map_err(|e| yams_core::Error::Database(e.to_string()))?)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?),
        country: row.get(1).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        postal_code: row.get(2).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        city: row.get(3).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        street: row.get(4).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        street_number: row.get(5).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        extra: row.get(6).map_err(|e| yams_core::Error::Database(e.to_string()))?,
    })
}
