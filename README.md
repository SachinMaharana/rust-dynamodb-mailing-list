# Rust DynamoDb Backed Mailing List

```
docker run -it --rm --name test-dynamodb -p 8000:8000 amazon/dynamodb-local
```

```
aws dynamodb create-table --cli-input-json file://news.json --endpoint-url http://localhost:8000 --region custom
```

## Usage
```
mod dynamo;

#[tokio::main]
async fn main() {
    let table_name = "news".to_string();
    let region = "us-east-1";

    let store = dynamo::Store::new(table_name, region);

    let newsletter = "blog".to_string();
    let email = "some_email".to_string();

    match store.add_subscriber(newsletter, email).await {
        Ok(_s) => (),
        Err(e) => println!("{:?}",e)
    }


    match  store.list_subscribers("blog".to_string()).await {
        Ok(_r) => (),
        Err(e) => println!("{:?}", e)
    }

    match  store.remove_subscriber(newsletter, email).await {
        Ok(_l) => (),
        Err(e) => println!("{:?}", e)
    }
}
```