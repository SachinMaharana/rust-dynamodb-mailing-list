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
    let store = dynamo::Store::new("news".to_string(), "us-east-1");

    match store.add_subscriber("blog".to_string(), "okay".to_string()).await {
        Ok(_s) => (),
        Err(e) => println!("{:?}",e)
    }


    match  store.list_subscribers("blog".to_string()).await {
        Ok(_r) => (),
        Err(e) => println!("{:?}", e)
    }

    match  store.remove_subscriber("blog".to_string(), "some_email".to_string()).await {
        Ok(_l) => (),
        Err(e) => println!("{:?}", e)
    }
}
```