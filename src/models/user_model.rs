use serde::{Serialize,Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize,FromRow,Debug)]
pub struct Usuario {
    pub id: String,
    pub name: String,
    pub public_key: String
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}