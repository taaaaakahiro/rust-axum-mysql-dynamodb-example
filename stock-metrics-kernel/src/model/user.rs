use chrono::{DateTime, Local};
use derive_new::new;



use super::Id;


#[derive(new, Debug)]
pub struct User {
    pub id: Id<User>,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new, Debug)]
pub struct NewUser {
    pub id: Id<User>,
    pub name: String,
}
