use axum::Router;

mod routes_login;
mod routes_hello;

// ---- Routes: Hello routes
pub fn api_test_routes() -> Router {
    Router::new()
        .merge(routes_hello::api_test_route_hello())
        .merge(routes_login::api_test_routes_login())
}

