extern crate actix_web as web;
extern crate env_logger;

use liboverflow::{
    models::AppContext,
    services::{
        GET_PROFILE, GET_ASSETS, GET_INDEX, GET_LOGIN, GET_REGISTER, GET_SCRIPTS, POST_LOGIN,
        POST_REGISTER,
    },
    sqlite3, api::{POST_IS_VALID_EMAIL, POST_IS_VALID_USERNAME},
};
use web::{App, HttpServer};

const ADDRESS: &str = "127.0.0.1:3000";

#[web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init().expect("Error initializing logger!");

    println!("Starting Server at: https://{}/", ADDRESS);

    HttpServer::new(|| {
        let conn = sqlite3::connection().expect("Error creating connection");
        App::new()
            .data(AppContext::create(conn))
            .service(GET_ASSETS)
            .service(GET_SCRIPTS)
            .service(GET_INDEX)
            .service(GET_REGISTER)
            .service(POST_REGISTER)
            .service(GET_LOGIN)
            .service(POST_LOGIN)
            .service(GET_PROFILE)
            .service(POST_IS_VALID_USERNAME)
            .service(POST_IS_VALID_EMAIL)
    })
    .bind_openssl(ADDRESS, liboverflow::ssl::builder()?)?
    .run()
    .await
}
