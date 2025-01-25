use axum::{response::Redirect, Router};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use super::{rest_api::RestApiServer, server_config::ServerConfig};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "OPENAPI", description = "Test description")
    )
)]
struct ApiDoc;

pub struct SwaggerServer;

impl ServerConfig for SwaggerServer {

    async fn start(ip: &str, port: u16) {
        let app: Router = Self::router();
        let address = format!("{ip}:{port}");
        let listener = tokio::net::TcpListener::bind(&address).await;
        
        match listener {
            Ok(v) => {
                Self::log(format!("Server started on port {port}"));
                axum::serve(v,app).await.unwrap();
            }
            Err(e) => {
                Self::log(format!("Error {e} on server {address}"));
            }
        }       
    }
    
    fn openapi_router() -> OpenApiRouter {
        return OpenApiRouter::with_openapi(ApiDoc::openapi())
            .nest("/api/v1/", RestApiServer::openapi_router());
    }

    fn router() -> Router {
        let (router, api) = Self::openapi_router().split_for_parts();
        return router
            .merge(
                SwaggerUi::new("/swagger-ui")
                    .url("/api-docs/openapi.json", api)
            )
            .fallback(Redirect::to("/swagger-ui"));
    }

    fn log(message: String) {
        println!("[{:>10}] {message}", "Swagger")
    }
}

