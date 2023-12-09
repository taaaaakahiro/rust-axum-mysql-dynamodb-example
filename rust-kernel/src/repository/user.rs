use async_trait::async_trait;

use crate::model::user::{NewUser, User};

#[async_trait]
pub trait UserRepository {
    async fn find_one(&self, id: &String) -> anyhow::Result<Option<User>>;
    async fn find(&self) -> anyhow::Result<Vec<User>>;
    async fn insert(&self, id: NewUser) -> anyhow::Result<String>;
    async fn delete_one(&self, id: &String) -> anyhow::Result<()>;
}
