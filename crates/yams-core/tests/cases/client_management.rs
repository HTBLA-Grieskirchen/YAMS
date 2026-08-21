use std::sync::Arc;

use crate::cases::TestAdapters;

use super::super::make_testing_app;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use yams_core::ports::RepositoryError;
use yams_core::{
    domain::Address,
};
use yams_core::service::CreateClient;

#[pollster::test]
async fn test_client() {
    let app = make_testing_app(&TestAdapters::default()).await;
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
        .map(|i| {
            let app_clone = app.clone();
            let mut case_clone = case.clone();
            case_clone.customer_number = 1000 + i;
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

    let clients = app
        .execute_fn::<_, _, RepositoryError>(async |ctx| {
            let mut clients = vec![ctx.uow.clients().find_by_id(result.id).await?];
            for res in parallel_results {
                clients.push(ctx.uow.clients().find_by_id(res.id).await?);
            }
            Ok(clients)
        })
        .await
        .unwrap();
    assert_eq!(clients.len(), 101);
    assert_eq!(clients[0].first_name, "Testname");
}
