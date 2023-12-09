use kernel::model::user::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonUser {
    pub id: String,
    pub name: String,
}

impl From<User> for JsonUser {
    fn from(s: User) -> Self {
        JsonUser {
            id: s.id,
            name: s.name,
        }
    }
}
