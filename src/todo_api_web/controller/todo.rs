use actix_web::{http::header::ContentType, post, HttpResponse, Responder};
use uuid::Uuid;

use crate::todo_api_web::model::todo::TodoIdResponse;

#[post("/api/create")]
pub async fn create_todo() -> impl Responder {
    let new_id = Uuid::new_v4();
    let str = serde_json::to_string(&TodoIdResponse::new(new_id)).unwrap();
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(str)
}