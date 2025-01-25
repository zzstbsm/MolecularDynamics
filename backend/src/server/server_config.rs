use axum::Router;
use utoipa_axum::router::OpenApiRouter;

pub trait ServerConfig {
    async fn start(address: &str, port: u16);
    fn openapi_router() -> OpenApiRouter;
    fn router() -> Router;
    fn log(message: String);
}
