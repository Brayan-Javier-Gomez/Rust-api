use crate::{
    models::message_model::{SendMessageRequest,UpdateStatusRequest},
    services::message_service,
};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn send_message(pool: web::Data<PgPool>, payload: web::Json<SendMessageRequest>) -> impl Responder {
    match message_service::new_message(&pool, &payload.sender_id,&payload.receiver_id, &payload.message).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"message": "Message send succesfully"})),
        Err(err) => {
            eprintln!("Error sending message: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({"message": "Failed to send message"}))
        }
    }
}

pub async fn get_pending_messages(pool: web::Data<PgPool>, receiver_id: web::Path<String>) -> impl Responder {
    match message_service::get_pending_messages(&pool,receiver_id.into_inner()).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(err) => {
            println!("Error al obtener usuarios: {:?}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}

pub async fn update_message_status(pool: web::Data<PgPool>, payload: web::Json<UpdateStatusRequest>) -> impl Responder {
    let status = &payload.status;
    let update_ids = &payload.message_ids;
    match message_service::update_message_status(&pool, &status,&update_ids).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"message": "Status update succesfully"})),
        Err(err) => {
            eprintln!("Error sending message: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({"message": "Failed to update status"}))
        }
    }
}
