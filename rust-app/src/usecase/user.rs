use std::sync::Arc;

use adapter::modules::RepositoriesModuleExt;
use derive_new::new;
use kernel::model::user::User;
use kernel::repository::user::UserRepository;

#[derive(new)]
pub struct UserUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub async fn find(&self) -> anyhow::Result<Vec<User>> {
        self.repositories.user_repo().find().await
    }

    pub async fn find_one(&self, _id: &String) -> anyhow::Result<Vec<User>> {
        self.repositories.user_repo().find().await
    }
}

#[cfg(test)]
mod test {
    use crate::usecase::user::UserUseCase;
    use adapter::modules::RepositoriesModule;
    use adapter::persistence::mysql::Db;
    use std::sync::Arc;

    #[tokio::test]
    async fn find() {
        let modules = RepositoriesModule::new(Db::new().await);
        let service = UserUseCase::new(Arc::new(modules));
        let got = service.find().await.unwrap();
        assert_eq!(got.len(), 4);
        assert_eq!(got[0].id, "userId1");
        assert_eq!(got[1].id, "userId2");
        assert_eq!(got[2].id, "userId3");
        assert_eq!(got[3].id, "userId4");
    }
}
