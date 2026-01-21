use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use yams_core::models::{Animal, NewAnimal, Race, NewRace};
use yams_core::ports::{AnimalRepository, RaceRepository};
use crate::adapter::SqliteAdapter;
use libsql::params;

#[async_trait]
impl AnimalRepository for SqliteAdapter {
    async fn find_all(&self) -> Result<Vec<Animal>, yams_core::Error> {
        let mut rows = self.db.query("SELECT id, name, birthdate, race_id FROM animals", ())
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let mut animals = Vec::new();
        while let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            animals.push(map_row_to_animal(&row)?);
        }
        Ok(animals)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Animal>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT id, name, birthdate, race_id FROM animals WHERE id = ?1", 
            params![id.to_string()]
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            Ok(Some(map_row_to_animal(&row)?))
        } else {
            Ok(None)
        }
    }

    async fn create(&self, animal: NewAnimal) -> Result<Animal, yams_core::Error> {
        let id = Uuid::new_v4();
        
        self.db.execute(
            "INSERT INTO animals (id, name, birthdate, race_id) 
             VALUES (?1, ?2, ?3, ?4)",
            params![
                id.to_string(),
                animal.name.clone(),
                animal.birthdate.to_rfc3339(),
                animal.race_id.to_string(),
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        Ok(Animal {
            id,
            name: animal.name,
            birthdate: animal.birthdate,
            race_id: animal.race_id,
        })
    }

    async fn update(&self, animal: Animal) -> Result<Animal, yams_core::Error> {
        self.db.execute(
            "UPDATE animals SET 
                name = ?2,
                birthdate = ?3,
                race_id = ?4
             WHERE id = ?1",
            params![
                animal.id.to_string(),
                animal.name.clone(),
                animal.birthdate.to_rfc3339(),
                animal.race_id.to_string(),
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        Ok(animal)
    }

    async fn delete(&self, id: Uuid) -> Result<(), yams_core::Error> {
        self.db.execute("DELETE FROM client_animals WHERE animal_id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        self.db.execute("DELETE FROM animals WHERE id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl RaceRepository for SqliteAdapter {
    async fn find_all(&self) -> Result<Vec<Race>, yams_core::Error> {
        let mut rows = self.db.query("SELECT id, description, animal_species FROM races", ())
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let mut races = Vec::new();
        while let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            races.push(map_row_to_race(&row)?);
        }
        Ok(races)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Race>, yams_core::Error> {
        let mut rows = self.db.query(
            "SELECT id, description, animal_species FROM races WHERE id = ?1", 
            params![id.to_string()]
        )
        .await
        .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if let Some(row) = rows.next()
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))? 
        {
            Ok(Some(map_row_to_race(&row)?))
        } else {
            Ok(None)
        }
    }

    async fn create(&self, race: NewRace) -> Result<Race, yams_core::Error> {
        let id = Uuid::new_v4();
        
        self.db.execute(
            "INSERT INTO races (id, description, animal_species) 
             VALUES (?1, ?2, ?3)",
            params![
                id.to_string(),
                race.description.clone(),
                race.animal_species.clone(),
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        Ok(Race {
            id,
            description: race.description,
            animal_species: race.animal_species,
        })
    }

    async fn update(&self, race: Race) -> Result<Race, yams_core::Error> {
        self.db.execute(
            "UPDATE races SET 
                description = ?2,
                animal_species = ?3
             WHERE id = ?1",
            params![
                race.id.to_string(),
                race.description.clone(),
                race.animal_species.clone(),
            ]
        ).await.map_err(|e| yams_core::Error::Database(e.to_string()))?;

        Ok(race)
    }

    async fn delete(&self, id: Uuid) -> Result<(), yams_core::Error> {
        // Check if any animals use this race
        let mut rows = self.db.query("SELECT count(*) FROM animals WHERE race_id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        let count: i64 = rows.next().await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?
            .ok_or_else(|| yams_core::Error::Database("Failed to get count".to_string()))?
            .get(0)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        
        if count > 0 {
            return Err(yams_core::Error::Validation("Cannot delete race used by animals".to_string()));
        }

        self.db.execute("DELETE FROM races WHERE id = ?1", params![id.to_string()])
            .await
            .map_err(|e| yams_core::Error::Database(e.to_string()))?;
        Ok(())
    }
}

fn map_row_to_animal(row: &libsql::Row) -> Result<Animal, yams_core::Error> {
    let id_str: String = row.get(0).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    let birthdate_str: String = row.get(2).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    let race_id_str: String = row.get(3).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    
    Ok(Animal {
        id: Uuid::parse_str(&id_str).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        name: row.get(1).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        birthdate: DateTime::parse_from_rfc3339(&birthdate_str)
            .map_err(|e| yams_core::Error::Database(e.to_string()))?
            .with_timezone(&Utc),
        race_id: Uuid::parse_str(&race_id_str).map_err(|e| yams_core::Error::Database(e.to_string()))?,
    })
}

fn map_row_to_race(row: &libsql::Row) -> Result<Race, yams_core::Error> {
    let id_str: String = row.get(0).map_err(|e| yams_core::Error::Database(e.to_string()))?;
    Ok(Race {
        id: Uuid::parse_str(&id_str).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        description: row.get(1).map_err(|e| yams_core::Error::Database(e.to_string()))?,
        animal_species: row.get(2).map_err(|e| yams_core::Error::Database(e.to_string()))?,
    })
}
