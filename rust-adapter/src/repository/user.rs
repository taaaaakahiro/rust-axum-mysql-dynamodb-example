use super::DatabaseRepositoryImpl;
use crate::model::user::UserTable;
use async_trait::async_trait;
use kernel::{
    model::user::{NewUser, User},
    repository::user::UserRepository,
};
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

    async fn find(&self) -> anyhow::Result<Vec<User>> {
        let pool = self.db.0.clone();
        let user_records = query_as::<_, UserTable>("select * from users ORDER BY id ASC")
            .fetch_all(&*pool)
            .await?;

        let users: Vec<User> = user_records
            .into_iter()
            .map(|u| User {
                id: u.id,
                name: u.name,
                created_at: u.created_at,
            })
            .collect();

        Ok(users)
    }

    async fn insert(&self, input: NewUser) -> anyhow::Result<String> {
        let pool = self.db.0.clone();
        let user_table: UserTable = input.try_into()?;
        let _ = sqlx::query("insert into users (id, name) values (?, ?)")
            .bind(&user_table.id)
            .bind(&user_table.name)
            .execute(&*pool)
            .await?;
        Ok(user_table.id.try_into()?)
    }

    async fn update(&self, user: NewUser) -> anyhow::Result<String> {
        let pool = self.db.0.clone();
        let user_table: UserTable = user.try_into()?;
        let _ = sqlx::query("update users set name = ? where id = ?")
            .bind(&user_table.name)
            .bind(&user_table.id)
            .execute(&*pool)
            .await?;

        Ok(user_table.id.try_into()?)
    }
}

#[cfg(test)]
mod test {
    use super::DatabaseRepositoryImpl;
    use crate::persistence::mysql::Db;
    use kernel::model::user::NewUser;
    use kernel::repository::user::UserRepository;

    #[tokio::test]
    async fn find_one() {
        let db = Db::new().await;
        let user_repo = DatabaseRepositoryImpl::new(db);

        let id = String::from("userId1");
        let got = user_repo
            .find_one(&id)
            .await
            .expect("failed to get users")
            .expect("user is not found");
        assert_eq!(got.id, id)
    }

    #[tokio::test]
    async fn find() {
        let db = Db::new().await;
        let user_repo = DatabaseRepositoryImpl::new(db);

        let got = user_repo.find().await.expect("failed to get");
        assert_eq!(got.len(), 4);
        assert_eq!(got[0].id, "userId1");
        assert_eq!(got[1].id, "userId2");
        assert_eq!(got[2].id, "userId3");
        assert_eq!(got[3].id, "userId4");
    }

    #[tokio::test]
    async fn test_insert() {
        let db = Db::new().await;
        let user_repo = DatabaseRepositoryImpl::new(db);

        let id = String::from("userId100");
        let user = NewUser {
            id: id.clone(),
            name: String::from("userName100"),
        };
        let got = user_repo.insert(user).await.expect("failed to insert user");

        assert_eq!(got, id.clone());
    }
}
