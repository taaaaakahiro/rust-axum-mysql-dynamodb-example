use crate::model::user_item::UserItem;
use async_trait::async_trait;

#[async_trait]
pub trait UserItemRepository {
    async fn find_one(&self, id: &String) -> anyhow::Result<Option<UserItem>>;
}
