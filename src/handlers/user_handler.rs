// use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};
// use base64::{engine::general_purpose, Engine};
use crate::{models::user_model::CreateUserRequest, services::user_service};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

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
        Ok((public_key, private_key)) => HttpResponse::Created()
            .json(serde_json::json!({"message": "User created successfully","user": &user.name,"public_key": public_key,"private_key": private_key})),
        Err(err) => {
            eprintln!("Error inserting user: {:?}", err);
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"message": "Failed to create user"}))
        }
    }
}

pub async fn update_user(
    pool: web::Data<sqlx::PgPool>,
    user_id: web::Path<String>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    match user_service::update_user(&pool, user_id.into_inner(), user_data.into_inner()).await {
        Ok(_) => HttpResponse::Ok()
            .json(serde_json::json!({"message": "User updated successfully"})),
        Err(err) => {
            println!("User update failed: {:?}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}

pub async fn get_user_by_id(
    pool: web::Data<sqlx::PgPool>,
    user_id: web::Path<String>,
) -> impl Responder {
    match user_service::get_user_by_id(&pool, user_id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            println!("User not found: {:?}", err);
            HttpResponse::NotFound().body(err)
        }
    }
}

// pub fn create_shared_key(sender_id_private_key:String, receiver_id_public_key:String) -> Result<String, String>{

//     let sender_private_key_bytes = match general_purpose::STANDARD.decode(sender_id_private_key) {
//         Ok(bytes) => bytes,
//         Err(_) => return Err("Error al decodificar la clave privada del emisor".to_string()),
//     };

//     let receiver_public_key_bytes = match general_purpose::STANDARD.decode(receiver_id_public_key) {
//         Ok(bytes) => bytes,
//         Err(_) => return Err("Error al decodificar la clave pública del receptor".to_string()),
//     };
//     let sender_private_key_array: [u8; 32] = sender_private_key_bytes
//         .try_into()
//         .map_err(|_| "La clave privada no tiene el tamaño correcto, se esperaban 32 bytes".to_string())?;

//     let receiver_public_key_array: [u8; 32] = receiver_public_key_bytes
//         .try_into()
//         .map_err(|_| "La clave pública no tiene el tamaño correcto, se esperaban 32 bytes".to_string())?;

//     // Convertir a tipos requeridos
//     let sender_private_key = StaticSecret::from(sender_private_key_array);
//     let receiver_public_key = PublicKey::from(receiver_public_key_array);

//     // Generar la clave compartida
//     let shared_key = sender_private_key.diffie_hellman(&receiver_public_key);
//     Ok(general_purpose::STANDARD.encode(shared_key.as_bytes()))
// }
