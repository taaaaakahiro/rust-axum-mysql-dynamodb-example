use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::{dynamodb::DynamoDB, mysql::Db};

pub mod health_check;
pub mod user;
mod user_item;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    db: Db,
    _marker: PhantomData<T>,
}

#[derive(new)]
pub struct DynamoDBRepositoryImpl<T> {
    dynamo_db: DynamoDB,
    _marker: PhantomData<T>,
}
