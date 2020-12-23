# Rust DynamoDb Backed Mailing List

```
docker run -it --rm --name test-dynamodb -p 8000:8000 amazon/dynamodb-local
```

```
aws dynamodb create-table --cli-input-json file://news.json --endpoint-url http://localhost:8000 --region custom
```