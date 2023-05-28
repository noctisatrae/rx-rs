use rxrs::{Infrastructure, Service};
use tokio::test as async_test;

#[async_test]
async fn infrastructure_run() {
    let service_a = Service::new("Hello", |test: &u32| 3 + test);

    let an_infrastructure = Infrastructure::<_, _, _> {
        services: vec![service_a],
    };

    let result = an_infrastructure.execute(3).await;
}
