use crate::{models::user_model::{CreateUserRequest, Usuario}, tools::gen_keys::generate_keys_open_ssl};
use sqlx::PgPool;
use std::result::Result;

pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<Usuario>, String> {
    sqlx::query_as::<_, Usuario>("SELECT id, name, public_key FROM users")
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

pub async fn insert_user(pool: &PgPool, name: &str) -> Result<(String,String), sqlx::Error> {
    println!("Creando usuario");
    let (public_key_encode, private_key) = match generate_keys_open_ssl() {
        Ok((private_key, public_key)) => {
            (public_key, private_key)
        }
        Err(err) => {
            eprintln!("Error al generar claves: {}", err);
            ("".to_string(), "".to_string())
        }
    };

    sqlx::query(
        "INSERT INTO users (name,public_key) VALUES ($1,$2)"
    ).bind(&name)
    .bind(&public_key_encode)
    .execute(pool)
    .await?;

    Ok((public_key_encode,private_key))
}

pub async fn update_user(
    pool: &PgPool,
    user_id: String,
    user_data: CreateUserRequest,
) -> Result<(), String> {
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

pub async fn get_user_by_id(pool: &PgPool, user_id: String) -> Result<Usuario, String> {
    let query = r#"
        SELECT id, name, public_key
        FROM users
        WHERE id = $1
    "#;

    sqlx::query_as::<_,Usuario>(query)
        .bind(&user_id.to_string())
        .fetch_one(pool)
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