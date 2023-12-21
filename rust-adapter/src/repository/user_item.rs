use crate::repository::DynamoDBRepositoryImpl;
use async_trait::async_trait;
use aws_sdk_dynamodb::model::AttributeValue;
use kernel::model::user_item::UserItem;
use kernel::repository::user_item::UserItemRepository;

#[async_trait]
impl UserItemRepository for DynamoDBRepositoryImpl<UserItem> {
    async fn get_item(&self, id: &String) -> anyhow::Result<Option<UserItem>> {
        match self
            .dynamo_db
            .client
            .get_item()
            .table_name("users")
            .key("id".to_string(), AttributeValue::N(id.to_string()))
            .send()
            .await
        {
            Ok(get_item_output) => {
                if let Some(item) = get_item_output.item {
                    println!("{:?}", item);

                    // Idを取り出す
                    if let Some(attr_val) = item.get("id") {
                        if let Ok(id_val) = attr_val.as_n() {
                            if let Ok(id) = id_val.parse::<u32>() {
                                println!("{}", id);
                            }
                        }
                    }

                    // Nameを取り出す
                    if let Some(attr_val) = item.get("name") {
                        if let Ok(name) = attr_val.as_s() {
                            println!("{}", name.to_string());
                        }
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn get_item() {
        //todo
        // let cli = init_client().await;
        // let db = DynamoDB::new(cli);
        // let repo = DynamoDBRepositoryImpl::new(db);
    }
}
