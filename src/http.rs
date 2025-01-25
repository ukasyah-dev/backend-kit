use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use serde::Serialize;
use serde_json::json;
use tokio::net::TcpListener;

use crate::{error, shutdown};

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(error::Error))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl From<JsonRejection> for error::Error {
    fn from(err: JsonRejection) -> Self {
        error::invalid_argument_with_message(&err.to_string())
    }
}

impl IntoResponse for error::Error {
    fn into_response(self) -> Response {
        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let payload = json!({
            "message": self.message,
        });

        (code, axum::Json(payload)).into_response()
    }
}

pub async fn serve(router: Router, port: u16) -> Result<(), axum::BoxError> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await.unwrap();

    log::info!("running http server on {}", listener.local_addr().unwrap());

    let r = Router::new()
        .merge(router)
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed);

    axum::serve(listener, r)
        .with_graceful_shutdown(shutdown::wait_for_signal())
        .await?;

    Ok(())
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, Json(json!({"message": "not found"}))).into_response()
}

async fn method_not_allowed() -> Response {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        Json(json!({"message": "method not allowed"})),
    )
        .into_response()
}
