mod common;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use common::make_testing_app;
use uuid::Uuid;
use yams_core::{domain::Address, service::client::CreateClient};

#[pollster::test]
async fn test_client() {
    let (app, adaptors) = make_testing_app().await;
    let app = Arc::new(app);

    let case = CreateClient {
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

    // Execute the case 100 times in parallel using threads
    let handles: Vec<_> = (0..100)
        .map(|_| {
            let app_clone = app.clone();
            let case_clone = case.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1));
                pollster::block_on(async move { app_clone.execute(case_clone).await.unwrap() })
            })
        })
        .collect();

    let result = app.execute(case).await.unwrap();
    // Wait for all threads to complete and collect results
    let parallel_results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    assert_eq!(parallel_results.len(), 100);

    println!("{:?}", result);

    let clients = adaptors
        .datastore
        .clients
        .lock()
        .unwrap()
        .values()
        .map(|v| v.cloned_data())
        .collect::<Vec<_>>();
    assert_eq!(clients.len(), 101);
    assert_eq!(clients[0].first_name, "Testname");
}
