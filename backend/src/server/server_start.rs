use super::{
    openapi::SwaggerServer,
    rest_api::RestApiServer,
    server_config::ServerConfig,
};

#[tokio::main]
pub async fn server_main(port: u16){
    
    let address: &str = "127.0.0.1";

    let (_main_server, _swagger_server) = tokio::join!(
        RestApiServer::start(address, port),
        SwaggerServer::start(address, port+1),
    );
}

