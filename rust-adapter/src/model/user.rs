use chrono::{DateTime, Local};
use kernel::model::user::User;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct UserTable {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
}

impl TryFrom<UserTable> for User {
    type Error = anyhow::Error;
    fn try_from(st: UserTable) -> Result<Self, Self::Error> {
        Ok(User::new(st.id, st.name, st.created_at))
    }
}
