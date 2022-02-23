use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};
use std::io::Read;

use crate::models::{AppContext, Person};

#[actix_web::post("/register")]
async fn post_register(
    data: Data<AppContext>,
    form: Form<Person>,
    _req: HttpRequest,
) -> impl Responder {
    match &data.connection.execute(
        "INSERT INTO person (username, email, password)
        VALUES (:username, :email, :password)",
        &[
            (":username", &form.username),
            (":email", &form.email),
            (":password", &form.password),
        ],
    ) {
        Ok(updated) => {
            println!("{} rows were updated", updated);
            HttpResponse::Found().header("Location", "/login").finish()
        }
        Err(err) => {
            println!("update failed: {}", err);
            HttpResponse::Found()
                .header("Location", "/register")
                .finish()
        }
    }
}

#[actix_web::get("/register")]
async fn get_register(_req: HttpRequest) -> impl Responder {
    if let Some(render) = std::fs::File::open("static/register.html")
        .ok()
        .and_then(|mut f| {
            let mut buf = Vec::new();
            if f.read_to_end(&mut buf).is_ok() {
                String::from_utf8(buf).ok()
            } else {
                None
            }
        })
    {
        HttpResponse::Ok().body(render)
    } else {
        HttpResponse::NoContent().body("")
    }
}
