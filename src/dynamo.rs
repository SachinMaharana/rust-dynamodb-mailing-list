use chrono::Utc;
use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, PutItemInput, QueryInput,
};

// #[derive(Debug)]
// struct Item {
//     newsletter: String,
//     email: String,
//     created_at: String,
// }

// impl Item {
//     fn _from_map(map: HashMap<String, AttributeValue>) -> Result<Item> {
//         let newsletter = map
//             .get("newsletter")
//             .ok_or_else(|| anyhow!("no newsletter present"))
//             .and_then(attr_to_string)?;
//         let email = map
//             .get("email")
//             .ok_or_else(|| anyhow!("no email present"))
//             .and_then(attr_to_string)?;
//         let created_at = map
//             .get("created_at")
//             .ok_or_else(|| anyhow!("no created timestamp present"))
//             .and_then(attr_to_string)?;

//         let item = Item {
//             newsletter,
//             email,
//             created_at,
//         };
//         Ok(item)
//     }
// }

pub struct Store {
    table_name: String,
    client: DynamoDbClient,
}

fn attr_to_string(attr: &AttributeValue) -> Result<String> {
    if let Some(value) = &attr.s {
        Ok(value.to_owned())
    } else {
        bail!("error in converting attribute value to string")
    }
}

fn string_to_attr(s: String) -> AttributeValue {
    AttributeValue {
        s: Some(s),
        ..Default::default()
    }
}

impl Store {
    pub fn new(table_name: String, region: &str) -> Self {
        let aws_region = match region.parse::<Region>() {
            Ok(r) => r,
            Err(e) => {
                println!("{:?}", e);
                std::process::exit(1);
            }
        };
        Store {
            table_name,
            client: DynamoDbClient::new(aws_region),
        }
    }

    pub async fn list_subscribers(&self, newsletter: String) -> Result<Vec<String>> {
        let expression = format!("newsletter = :newsletter");
        let mut values = HashMap::new();
        values.insert(":newsletter".into(), string_to_attr(newsletter));

        let query = QueryInput {
            table_name: self.table_name.clone(),
            key_condition_expression: Some(expression),
            expression_attribute_values: Some(values),
            ..Default::default()
        };
        let items = self
            .client
            .query(query)
            .await?
            .items
            .ok_or_else(|| anyhow!("Error in Items"))?;

        let mut emails = Vec::new();

        for i in items {
            if let Some(email) = i.get("email") {
                let e = attr_to_string(email)?;
                emails.push(e);
            }
        }
        Ok(emails)
    }

    pub async fn remove_subscriber(&self, newsletter: String, email: String) -> Result<()> {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert("newsletter".into(), string_to_attr(newsletter));
        key.insert("email".into(), string_to_attr(email));

        let query = DeleteItemInput {
            table_name: self.table_name.clone(),
            key,
            ..Default::default()
        };

        match self.client.delete_item(query).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub async fn add_subscriber(&self, newsletter: String, email: String) -> Result<()> {
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        let created_at = Utc::now().to_string();

        item.insert("newsletter".into(), string_to_attr(newsletter));
        item.insert("email".into(), string_to_attr(email));
        item.insert("created_at".into(), string_to_attr(created_at));

        let dynamo_db_item = PutItemInput {
            table_name: self.table_name.clone(),
            item,
            ..Default::default()
        };
        match self.client.put_item(dynamo_db_item).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
