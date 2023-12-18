use crate::repository::DynamoDBRepositoryImpl;
use async_trait::async_trait;
use kernel::model::user_item::UserItem;
use kernel::repository::user_item::UserItemRepository;

#[async_trait]
impl UserItemRepository for DynamoDBRepositoryImpl<UserItem> {
    async fn find_one(&self, _id: &String) -> anyhow::Result<Option<UserItem>> {
        todo!()
    }
}
