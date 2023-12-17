use std::env;
use std::sync::Arc;

use aws_config::load_from_env;
use aws_sdk_dynamodb::config::Builder;
use aws_sdk_dynamodb::{Client, Endpoint, Region};
use http::Uri;

pub struct DynamoDB {
    pub(crate) client: Arc<Client>,
}

pub async fn init_client() -> Client {
    let config = load_from_env().await;
    let region = Region::from_static("ap-northeast-");

    let dynamodb_uri = env::var("DYNAMODB_URI").expect("DYNAMODB_URI is not defined");
    let static_dynamodb_uri: &'static str = Box::leak(dynamodb_uri.into_boxed_str());
    let config = Builder::from(&config)
        .region(region)
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(static_dynamodb_uri)))
        .build();
    let dynamodb = Client::from_conf(config);
    dynamodb
}

impl DynamoDB {
    pub fn new(client: Client) -> DynamoDB {
        DynamoDB {
            client: Arc::new(client),
        }
    }

    pub async fn list_table_names(&self) -> anyhow::Result<Option<Vec<String>>> {
        let res = self.client.list_tables().send().await?;
        Ok(res.table_names)
    }
}

#[cfg(test)]
mod test {
    use super::{init_client, DynamoDB};

    #[tokio::test]
    async fn hc_list_table_names() {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let _ = dynamodb
            .list_table_names()
            .await
            .expect("failed to get tables")
            .unwrap();
    }
}
