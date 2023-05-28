use rxrs::Service;
use tokio::test as async_test;

#[async_test]
async fn service_closure() {
    let a_service = Service::new("Hello", |a: &u32| 3 + a);

    assert_eq!(a_service.call(&3).await, 6);
}
