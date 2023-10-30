use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Local>,
}
