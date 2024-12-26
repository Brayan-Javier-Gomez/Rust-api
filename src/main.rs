mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;
mod tools;


use actix_web::{web, App, HttpServer};
use config::server_config::host_config;
use db::db_conection::configure_database;
use routes::config_routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (server_ip, server_port) = host_config();
    let pool = configure_database().await;
    println!("Servidor iniciado en http://{}:{}", server_ip, server_port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default())
            .configure(config_routes)
    })
    .bind((server_ip.as_str(), server_port))?
    .run()
    .await
}
