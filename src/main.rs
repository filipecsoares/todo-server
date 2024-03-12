mod todo_api_web;

use todo_api_web::routes::app_routes;
use actix_web::{App, HttpServer};
use num_cpus;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(||{
       App::new().configure(app_routes)  
   })
   .workers(num_cpus::get() + 2)
   .bind(("localhost", 4000))
   .unwrap()
   .run()
   .await
}
