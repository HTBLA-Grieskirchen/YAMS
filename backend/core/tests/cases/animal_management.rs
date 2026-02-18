use std::sync::Arc;

use super::super::make_testing_app;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use yams_core::{
    domain::Address,
    service::{
        UseCase,
        animals::{CreateAnimal, CreateManyAnimals},
        client::CreateClient,
    },
};

#[pollster::test]
async fn test_animal() {
    let (app, adaptors) = make_testing_app().await;
    let app = Arc::new(app);

    let create_client = CreateClient {
        first_name: "Testname".into(),
        last_name: "Testname Last".into(),
        birthdate: Utc::now().date_naive(),
        email: "test@test.com".into(),
        mobile_number: "1234567890".into(),
        customer_number: 1234567890,
        consent: false,
        address: Address {
            postal_code: "12345".into(),
            city: "Testcity".into(),
            street_and_number: "Teststreet 12".into(),
            country_code: "DE".into(),
        },
    };

    let client = app.execute(create_client).await.unwrap();
    let animal_amount = 10;

    // Execute the case 10 times in parallel using threads
    let results: Vec<_> = (0..animal_amount)
        .map(|i| {
            let app_clone = app.clone();
            let client_id = client.id.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1));
                pollster::block_on(async move {
                    let add_animal = CreateAnimal {
                        client_id: client_id,
                        name: format!("Testanimal {}", i).into(),
                        birthdate: Utc::now().date_naive(),
                        animal_species: "Testspecies".into(),
                        description: "Testdescription".into(),
                    };
                    app_clone.execute(add_animal).await.unwrap()
                })
            })
        })
        .map(|handle| handle.join().unwrap())
        .collect();

    assert_eq!(results.len(), animal_amount);

    let cmd = CreateManyAnimals {
        animals: (0..animal_amount)
            .map(|i| CreateAnimal {
                client_id: client.id.clone(),
                name: format!("Testanimal {}", i).into(),
                birthdate: Utc::now().date_naive(),
                animal_species: "Testspecies".into(),
                description: "Testdescription".into(),
            })
            .collect(),
    };

    let results = app.execute(cmd).await.unwrap();
    assert_eq!(results.len(), 10);

    let client = app
        .execute_fn(async |ctx| ctx.uow.clients().find_by_id(client.id).await)
        .await
        .unwrap();
    assert_eq!(client.animal_ids.len(), animal_amount * 2);
}
