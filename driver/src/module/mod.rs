use adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::{
        dynamodb::{init_client, DynamoDB},
        mysql::Db,
    },
    repository::health_check::HealthCheckRepository,
};
use app::usecase::health_check::HealthCheckUseCase;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase {
        &self.health_check_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);

        let health_check_use_case =
            HealthCheckUseCase::new(HealthCheckRepository::new(db, dynamodb));

        Self {
            health_check_use_case,
        }
    }
}
