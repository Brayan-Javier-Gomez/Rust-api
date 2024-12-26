pub mod user_route;
pub mod auth_route;
pub mod index_route;
pub mod message_route;

use actix_web::web::ServiceConfig;

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(user_route::user_routes());
    cfg.service(index_route::index_routes());
    cfg.service(message_route::message_routes());
    // cfg.service(auth::auth_routes()); // Descomenta si tienes más rutas
}