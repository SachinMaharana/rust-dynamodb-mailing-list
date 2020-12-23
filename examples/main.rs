mod dynamo;

#[tokio::main]
async fn main() {
    let store = dynamo::Store::new("news".to_string(), "us-east-1");

    match store.add_subscriber("blog".to_string(), "some_email".to_string()).await {
        Ok(_s) => (),
        Err(e) => println!("{:?}",e)
    }


    match  store.list_subscribers("blog".to_string()).await {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e)
    }

    match  store.remove_subscriber("blog".to_string(), "some_email".to_string()).await {
        Ok(_l) => (),
        Err(e) => println!("{:?}", e)
    }
}
