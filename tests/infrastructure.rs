use rxrs::{Infrastructure, Service, ServiceResult};
use tokio::{test as async_test};
use std::sync::Arc;

#[async_test]
async fn infrastructure_run() {
    let service_a = Service::new("Add 3", Box::new(|test: &u32| {
        3 + test
    } ));
    let service_b = Service::new("Substract 2", Box::new(|test: &u32| test - 2));

    let mut an_infrastructure = Infrastructure {
        services: vec![service_a, service_b],
        result: ServiceResult { 
            iteration: 0, 
            previous: vec![], 
            current: Arc::new(3) 
        }
    };

    let result = an_infrastructure.execute().await;

    assert_eq!(result.current, Arc::new(4))
}
