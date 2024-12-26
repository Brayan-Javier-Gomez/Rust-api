use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

pub async fn configure_database() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurada");
    let retry_strategy = ExponentialBackoff::from_millis(100).map(jitter).take(5); // Intentos 5 veces con backoff exponencial

    Retry::spawn(retry_strategy, || {
        PgPoolOptions::new()
            .max_connections(5) // Número máximo de conexiones
            .connect(&database_url)
    })
    .await
    .expect("Error al intentar conectar con la base de datos después de varios intentos")
}
