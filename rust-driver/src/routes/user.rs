use crate::model::user::JsonUser;
use crate::module::{Modules, ModulesExt};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use tracing::error;

pub async fn user_find(
    Extension(modules): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.user_use_case().find().await;
    match res {
        Ok(sv) => {
            let json: Vec<JsonUser> = sv
                .into_iter()
                .map(|user| JsonUser {
                    id: user.id,
                    name: user.name,
                })
                .collect();
            Ok((StatusCode::OK, Json(json)))
        }
        Err(err) => {
            error!("Unexpected error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
