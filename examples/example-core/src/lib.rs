mod html;
mod i18n;

use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use dotenvy::dotenv;
use rcfe::{Client, DefaultClient};
use serde::Serialize;
use std::sync::{Arc, Once};

#[cfg(feature = "tracing-subscriber")]
use tracing_subscriber;

pub use html::{HtmlMetadata, HtmlTemplate, I18nMetadata};
pub use i18n::{Lang, t, DEFAULT_LANG};

#[derive(Clone)]
pub struct AppState<C>
where
    C: Client,
{
    pub client: C,
}

impl<C> AppState<C>
where
    C: Client,
{
    pub fn new(client: C) -> Self {
        AppState { client }
    }
}

#[derive(Clone)]
pub struct DefaultAppState {
    pub client: DefaultClient,
}

impl DefaultAppState {
    pub fn new(client: DefaultClient) -> Self {
        DefaultAppState { client }
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub enum AppError {
    Rcfe(rcfe::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }
        let (status, message, err) = match &self {
            AppError::Rcfe(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("RCFE Error: {}", e),
                Some(self),
            ),
        };

        let mut response = (status, AppJson(ErrorResponse { message })).into_response();
        if let Some(err) = err {
            // Insert our error into the response, our logging middleware will use this.
            // By wrapping the error in an Arc we can use it as an Extension regardless of any inner types not deriving Clone.
            response.extensions_mut().insert(Arc::new(err));
        }
        response
    }
}

impl From<rcfe::Error> for AppError {
    fn from(value: rcfe::Error) -> Self {
        AppError::Rcfe(value)
    }
}

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        dotenv().ok();
        // 初始化日志系统
        #[cfg(feature = "tracing-subscriber")]
        {
            tracing_subscriber::fmt::init();
        }
    });
}

/// 获取 ETCD 的 endpoints 列表 从环境变量 ETCD_ENDPOINTS 中读取 如果没有设置则使用默认值 http://localhost:2379
pub fn get_endpoints() -> Vec<String> {
    std::env::var("ETCD_ENDPOINTS")
        .unwrap_or_else(|_| "http://localhost:2379".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

/// 获取 ETCD 的命名空间 从环境变量 ETCD_NAMESPACE 中读取 如果没有设置则返回 None
pub fn get_namespace() -> Option<String> {
    std::env::var("ETCD_NAMESPACE").ok()
}
