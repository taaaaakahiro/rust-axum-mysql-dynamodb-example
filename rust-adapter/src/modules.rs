use crate::persistence::mysql::Db;
use crate::repository::DatabaseRepositoryImpl;
use kernel::model::user::User;
use kernel::repository::user::UserRepository;

pub struct RepositoriesModule {
    user_repo: DatabaseRepositoryImpl<User>,
}

pub trait RepositoriesModuleExt {
    type UserRepo: UserRepository;

    fn user_repo(&self) -> &Self::UserRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type UserRepo = DatabaseRepositoryImpl<User>;

    fn user_repo(&self) -> &Self::UserRepo {
        &self.user_repo
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        Self {
            user_repo: DatabaseRepositoryImpl::new(db.clone()),
        }
    }
}
