use derive_new::new;
use std::marker::PhantomData;
use ulid::Ulid;

pub mod user;
pub mod user_item;

#[derive(new, Debug, Clone, Copy)]
pub struct Id<T> {
    pub value: Ulid,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn gen() -> Id<T> {
        Id::new(Ulid::new())
    }
}
