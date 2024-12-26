use actix_web::{web, HttpResponse, Scope};

pub fn index_routes() -> Scope {
    web::scope("/")
        .route("", web::get().to(|| async { HttpResponse::Ok().body("Home") }))
}