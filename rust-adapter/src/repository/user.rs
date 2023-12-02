use super::DatabaseRepositoryImpl;
use crate::model::user::UserTable;
use async_trait::async_trait;
use kernel::{model::user::User, repository::user::UserRepository};
use sqlx::query_as;

#[async_trait]
impl UserRepository for DatabaseRepositoryImpl<User> {
    async fn find_one(&self, id: &String) -> anyhow::Result<Option<User>> {
        let pool = self.db.0.clone();
        let user_table = query_as::<_, UserTable>("select * from users where id = ?")
            .bind(id.to_string())
            .fetch_one(&*pool)
            .await
            .ok();
        match user_table {
            Some(st) => Ok(Some(st.try_into()?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use super::DatabaseRepositoryImpl;
    use crate::persistence::mysql::Db;
    use kernel::repository::user::UserRepository;

    #[tokio::test]
    async fn test_find_user() {
        let db = Db::new().await;
        let repository = DatabaseRepositoryImpl::new(db);

        let id = String::from("1");
        let _got = repository.find_one(&id).await.expect("failed to get");
        //todo
    }
}
