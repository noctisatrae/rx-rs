                                                                 
8 888888888o.   `8.`8888.      ,8' 8 888888888o.     d888888o.   
8 8888    `88.   `8.`8888.    ,8'  8 8888    `88.  .`8888:' `88. 
8 8888     `88    `8.`8888.  ,8'   8 8888     `88  8.`8888.   Y8 
8 8888     ,88     `8.`8888.,8'    8 8888     ,88  `8.`8888.     
8 8888.   ,88'      `8.`88888'     8 8888.   ,88'   `8.`8888.    
8 888888888P'       .88.`8888.     8 888888888P'     `8.`8888.   
8 8888`8b          .8'`8.`8888.    8 8888`8b          `8.`8888.  
8 8888 `8b.       .8'  `8.`8888.   8 8888 `8b.    8b   `8.`8888. 
8 8888   `8b.    .8'    `8.`8888.  8 8888   `8b.  `8b.  ;8.`8888 
8 8888     `88. .8'      `8.`8888. 8 8888     `88. `Y8888P ,88P' 

A reactive programming crate written in Rust! 

Install using the git link!
rxrs = { git = "https://github.com/noctisatrae/rx-rs.git" }

Exemple usage from tests/infrastructure.rs:

#[async_test]
async fn infrastructure_run() {
    let service_a = Service::new("Add 3", Box::new(|test: &u32| 3 + test));
    let service_b = Service::new("Substract 2", Box::new(|test: &u32| test - 2));

    let mut an_infrastructure = Infrastructure {
        services: vec![service_a, service_b],
        result: ServiceResult {
            iteration: 0,
            previous: vec![],
            current: Arc::new(3),
        },
    };

    let result = an_infrastructure.execute().await;

    assert_eq!(result.current, Arc::new(4))
}

- define your services and what they do...
- define an infrastructure containing your services...
- initialize the iterator so you can keep track of your data
- pass a vector to the "previous" field that will contain the value of each iteration
- create your base value so your infrastructure has something to work with.
- start your infrastructure and wait for the result!
- assert_eq to see if it works as intented.