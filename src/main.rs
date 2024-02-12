mod actions;
mod db;
mod models;
mod routes;
mod schema;

use actix_web::{web, App, HttpServer};
use db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api/subscription").configure(routes::configure_routes))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
