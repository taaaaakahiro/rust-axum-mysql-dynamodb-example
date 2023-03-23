use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::{errors::AppError};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(_) => {
                let msg = format!("Input validation error: [{}]", self).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::JsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}

// #[async_trait]
// impl<T, B> FromRequest<B> for ValidatedRequest<T>
// where
//     T: DeserializeOwned + Validate,
//     B: http_body::Body + Send,
//     B::Data: Send,
//     B::Error: Into<BoxError>,
// {
//     type Rejection = AppError;

//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>::from_request(req).await?;
//         value.validate()?;
//         Ok(ValidatedRequest(value))
//     }
// }
