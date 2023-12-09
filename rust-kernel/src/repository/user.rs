use async_trait::async_trait;

use crate::model::user::User;

#[async_trait]
pub trait UserRepository {
    async fn find_one(&self, id: &String) -> anyhow::Result<Option<User>>;
    async fn find(&self) -> anyhow::Result<Vec<User>>;
    async fn delete_one(&self, id: &String) -> anyhow::Result<()>;
}
