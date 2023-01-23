#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use actix_redis::RedisSession;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod api_error;
mod db;
mod post;
mod schema;
mod user;
mod util;

async fn index() -> impl Responder {
    "Welcome!".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    db::init();

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST not set");
    let redis_port = env::var("REDIS_PORT").expect("REDIS PORT not set");

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(RedisSession::new(format!("{}:{}", redis_host, redis_port), &[0; 32]).ttl(3600))
            .configure(user::init_routes)
            .configure(post::init_routes)
            .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let port = env::var("PORT").expect("HOST not set");
            let host = env::var("HOST").expect("HOST not set");
            server.bind(&format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}
