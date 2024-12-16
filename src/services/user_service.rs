use sqlx::PgPool;
use std::result::Result;
use crate::models::user_model::{CreateUserRequest, Usuario};

pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<Usuario>, String> {
    sqlx::query_as::<_, Usuario>("SELECT id, name FROM users")
        .fetch_all(pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(db_error) => {
                println!("Error de base de datos: {:?}", db_error);
                format!("Error de base de datos: {:?}", db_error)
            }
            _ => {
                println!("Otro error: {:?}", err);
                "Error desconocido al obtener usuarios".to_string()
            }
        })
}

pub async fn insert_user(pool: &PgPool, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (name) VALUES ($1)",
        name
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_user(pool: &PgPool, user_id: String, user_data: CreateUserRequest) -> Result<(), String> {
    sqlx::query!(
        "UPDATE users SET name = $1 WHERE id = $2",
        user_data.name,
        user_id
    )
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