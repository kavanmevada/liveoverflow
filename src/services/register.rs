use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};
use std::io::Read;

use crate::{
    models::{AppContext, Person},
    sqlite3::{self, Registration},
};

#[actix_web::post("/register")]
async fn post_register(
    data: Data<AppContext>,
    form: Form<Person>,
    _req: HttpRequest,
) -> impl Responder {
    let ret = sqlite3::register(&data.connection, form.0);

    match ret {
        Registration::Success(person) => {
            dbg!(person);
            HttpResponse::Found().header("Location", "/login").finish()
        }
        _ => HttpResponse::Found()
            .header("Location", "/register")
            .finish(),
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
