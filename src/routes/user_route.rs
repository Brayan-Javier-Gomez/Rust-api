use actix_web::{web, Scope};
use crate::handlers::user_handler::{get_all_users,update_user,create_user};

pub fn user_routes() -> Scope {
    web::scope("/usuarios")
        .route("/", web::get().to(get_all_users))  // Obtener todos los usuarios
        .route("", web::get().to(get_all_users))  // Obtener todos los usuarios
        .route("", web::post().to(create_user))   // Crear un nuevo usuario
        .route("/{id}", web::put().to(update_user))
}