use utoipa_axum::router::OpenApiRouter;

mod routes_login;
mod routes_hello;

pub const TAG_TEST: &str = "TAG_TEST";

// ---- Routes: Hello routes
pub fn api_test_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .merge(routes_hello::api_test_route_hello())
        .merge(routes_login::api_test_routes_login())
}

