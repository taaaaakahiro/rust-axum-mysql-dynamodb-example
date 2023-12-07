use chrono::{DateTime, Local};
use kernel::model::user::{NewUser, User};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct UserTable {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
}

impl UserTable {
    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl TryFrom<UserTable> for User {
    type Error = anyhow::Error;
    fn try_from(ut: UserTable) -> Result<Self, Self::Error> {
        Ok(User::new(ut.id, ut.name, ut.created_at))
    }
}

impl TryFrom<NewUser> for UserTable {
    type Error = anyhow::Error;
    fn try_from(nu: NewUser) -> Result<Self, Self::Error> {
        Ok(UserTable {
            id: nu.id,
            name: nu.name,
            created_at: Local::now(),
        })
    }
}
