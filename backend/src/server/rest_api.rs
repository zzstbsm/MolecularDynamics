use axum::{http::StatusCode, response::IntoResponse, Router};
use utoipa_axum::router::OpenApiRouter;

use super::server_config::ServerConfig;

pub mod simulation_management;
pub mod simulation_results;

pub mod test_routes;

pub struct RestApiServer;

impl ServerConfig for RestApiServer {

    async fn start(ip: &str, port: u16) {
        let app: Router = Self::router()
            .fallback(fallback_server);

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
        
        return OpenApiRouter::new()
            .merge(test_routes::api_test_routes())
            .merge(simulation_management::api_simulation_management())
            .merge(simulation_results::api_simulation_results())
    }

    fn router() -> Router {
        let (router, _) = Self::openapi_router().split_for_parts();
        return router;
    }

    fn log(message: String) {
        println!("[{:>10}] {message}", "server")
    }
}

async fn fallback_server() -> impl IntoResponse {
    return (StatusCode::NOT_FOUND, "Endpoint not present");
}
