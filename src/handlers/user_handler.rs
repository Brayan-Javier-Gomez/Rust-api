use actix_web::{HttpResponse, Responder, web};
use crate::{models::user_model::CreateUserRequest, services::user_service};
use sqlx:: PgPool;

pub async fn get_all_users(pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match user_service::fetch_all_users(&pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            println!("Error al obtener usuarios: {:?}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    match user_service::insert_user(&pool, &user.name).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"message": "User created successfully"})),
        Err(err) => {
            eprintln!("Error inserting user: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({"message": "Failed to create user"}))
        }
    }
}

pub async fn update_user(
    pool: web::Data<sqlx::PgPool>,
    user_id: web::Path<String>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    match user_service::update_user(&pool, user_id.into_inner(), user_data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Usuario actualizado correctamente"})),
        Err(err) => {
            println!("Error al actualizar usuario: {:?}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}
// pub async fn create_user(new_user: web::Json<Usuario>, pool: web::Data<PgPool>) -> HttpResponse {
//     let user = create_user_service(new_user.into_inner(), pool).await;
//     match user {
//         Ok(user) => HttpResponse::Created().json(user),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
//     }
// }
