use actix_web::{web, Scope};
use crate::handlers::message_handler::{send_message,get_pending_messages,update_message_status};

pub fn message_routes() -> Scope {
    web::scope("/chat")
        .route("/send", web::post().to(send_message))
        .route("/pending/{id}", web::get().to(get_pending_messages))
        .route("/update_status", web::patch().to(update_message_status))

}