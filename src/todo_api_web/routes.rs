use actix_web::{web, HttpResponse};
use crate::todo_api_web::controller::{
    ping, readiness,
    todo::create_todo
};

pub fn app_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(
                web::scope("/api")
                    .service(create_todo)
            )
            .service(ping)
            .service(readiness)
            .default_service(web::to(|| HttpResponse::NotFound())),
    );
}