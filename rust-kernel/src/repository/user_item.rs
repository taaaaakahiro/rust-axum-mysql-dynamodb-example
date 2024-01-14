use crate::model::user_item::UserItem;
use async_trait::async_trait;

#[async_trait]
pub trait UserItemRepository {
    async fn get_by_id(&self, id: &String) -> anyhow::Result<Option<UserItem>>;
}
