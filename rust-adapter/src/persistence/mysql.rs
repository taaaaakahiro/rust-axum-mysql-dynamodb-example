use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Db {
        let pool = MySqlPoolOptions::new()
            .max_connections(8)
            .connect(&env::var("MYSQL_DSN").unwrap_or_else(|_| panic!("MYSQL_DSN must be set!")))
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });
        Db(Arc::new(pool))
    }
}
