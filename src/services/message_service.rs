use base64::{engine::general_purpose, Engine};
use openssl::rsa;
use sqlx::PgPool;
use std::result::Result;

use crate::{models::message_model::PendingMessage, tools::encription::{encode_base64, encrypt_message_with_openssl}};

//Nuevo mensaje
pub async fn new_message(
    pool: &PgPool,
    sender_id: &str,
    receiver_id: &str,
    message: &str,
    public_key: &String
) -> Result<(), String> {
    let public_key_decoded = general_purpose::STANDARD.decode(public_key).map_err(|e| format!("Error al decodificar la clave pública desde DER: {}", e))?;
    let public_key_decripted = rsa::Rsa::public_key_from_der(&public_key_decoded).map_err(|e| format!("Error al crear la clave pública desde DER: {}", e))?;
    let encrypted_message = encrypt_message_with_openssl(public_key_decripted, message).map_err(|e| format!("Error al crear la clave pública desde DER: {}", e))?;
    let encode_encrypted_message = encode_base64(&encrypted_message);
    let query = r#"
        INSERT INTO messages (sender_id, receiver_id, message, status)
        VALUES ($1, $2, $3, 'pending')
        RETURNING id
    "#;

    sqlx::query(&query)
        .bind(&sender_id)
        .bind(&receiver_id)
        .bind(&encode_encrypted_message)
        .execute(pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(db_error) => {
                println!("Error de base de datos: {:?}", db_error);
                format!("Error de base de datos: {:?}", db_error)
            }
            _ => {
                println!("Otro error: {:?}", err);
                "Error desconocido al actualizar usuario".to_string()
            }
        })?;

    Ok(())
}

//Obtener mensajes pendientes
pub async fn get_pending_messages(
    pool: &PgPool,
    receiver_id: String,
) -> Result<Vec<PendingMessage>, String> {

    let query = r#"
        SELECT id, sender_id, message, date
        FROM messages
        WHERE receiver_id = $1 AND status = 'pending'
        ORDER BY date ASC
    "#;

    sqlx::query_as::<_, PendingMessage>(query)
        .bind(&receiver_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(db_error) => {
                println!("Error de base de datos: {:?}", db_error);
                format!("Error de base de datos: {:?}", db_error)
            }
            _ => {
                println!("Otro error: {:?}", err);
                "Error desconocido al actualizar usuario".to_string()
            }
        })
}

//Actualizar estado de mensajes
pub async fn update_message_status(pool: &PgPool, status:&str, update_ids: &Vec<String>) -> Result<(), String> {
    let query = r#"
        UPDATE messages
        SET status = $1
        WHERE id = ANY($2)
    "#;
    sqlx::query(query)
        .bind(status)
        .bind(update_ids)
        .execute(pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(db_error) => {
                println!("Error de base de datos: {:?}", db_error);
                format!("Error de base de datos: {:?}", db_error)
            }
            _ => {
                println!("Otro error: {:?}", err);
                "Error desconocido al actualizar usuario".to_string()
            }
        })?;

    Ok(())
}
