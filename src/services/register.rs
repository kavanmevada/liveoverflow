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

        Registration::Failed(_error) => {
            //HttpResponse::Found().header("Content-Type", "application/json").body("{ body: \"This is the error\" }")
            let mut buffer = String::new();
            let page = std::fs::File::open("static/register.html").and_then(|mut f| f.read_to_string(&mut buffer));
            if page.is_ok() {
                HttpResponse::Ok().body(buffer.replace(
                    r#"<p class="help is-danger is-hidden" id="username-error-msg"></p>"#, 
                    r#"<p class="help is-danger" id="username-error-msg">This email is invalid</p>"#
                ))
            } else {
                HttpResponse::NoContent().finish()
            }
        }
        _ => HttpResponse::Found()
            .header("Location", "/register")
            .finish(),
    }
}

#[actix_web::get("/register")]
async fn get_register(_req: HttpRequest) -> impl Responder {
    let mut buffer = String::new();
    let page = std::fs::File::open("static/register.html").and_then(|mut f| f.read_to_string(&mut buffer));
    if page.is_ok() {
        HttpResponse::Ok().body(buffer)
    } else {
        HttpResponse::NoContent().finish()
    }
}
