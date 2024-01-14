use derive_new::new;

#[derive(new, Debug)]
pub struct UserItem {
    pub id: String,
    pub name: String,
}
