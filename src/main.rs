use actix_web::{HttpServer, App};
use actix_web::{get, Responder, HttpResponse, web};

#[get("/ping")]
pub async fn ping() -> impl Responder {
   HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(||{
       App::new()
           .service(ping)
           .default_service(web::to(||HttpResponse::NotFound()))    
   })
   .workers(6)
   .bind(("localhost", 4000))
   .unwrap()
   .run()
   .await
}
