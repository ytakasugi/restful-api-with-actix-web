#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use actix_web::{middleware, App, HttpServer};

mod api;
mod util;
mod schema;
mod model;
mod logger;

use api::{get_api, post_api};
use util::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();

    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_api::get_task)
            .service(get_api::get_all_user)
            .service(get_api::get_all_task)
            .service(get_api::get_user_all_task)
            .service(post_api::new_user)
            .service(post_api::new_task)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}