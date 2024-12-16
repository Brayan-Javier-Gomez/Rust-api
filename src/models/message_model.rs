use serde::{Serialize,Deserialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub sender_id: String,
    pub receiver_id: String,
    pub message: String,
}

#[derive(Serialize,FromRow)]
pub struct PendingMessage {
    pub id: String,
    pub sender_id: String,
    pub message: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub message_ids: Vec<String>,
    pub status: String, // 'delivered' o 'read'
}