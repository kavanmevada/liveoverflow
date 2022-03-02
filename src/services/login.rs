use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};

use std::{fs::File, io::Read};

use crate::{
    models::{AppContext, Login},
    sqlite3::{self, LoginType},
};

#[actix_web::post("/login")]
async fn post_login(data: Data<AppContext>, form: Form<Login>, _req: HttpRequest) -> HttpResponse {
    let ret = sqlite3::login(
        &data.connection,
        LoginType::Traditional(&form.username, &form.password),
    );

    let mut buffer = String::new();
    let page = File::open("static/profile.html").and_then(|mut f| f.read_to_string(&mut buffer));

    match ret {
        sqlite3::Login::Success(person) if page.is_ok() => {
            //let replace = buffer.replace("/*UUID*/", &uuid);
            HttpResponse::Found()
                .header(
                    "Set-Cookie",
                    format!("__Secure-id={}; Secure; Max-Age=10; SameSite=Strict", &person.sessionid),
                )
                .header("Location", "/profile")
                .finish()
        }

        _ => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::get("/login")]
async fn get_login(_req: HttpRequest) -> impl Responder {
    let mut buffer = String::new();
    let page = File::open("static/login.html").and_then(|mut f| f.read_to_string(&mut buffer));
    if page.is_ok() {
        HttpResponse::Ok().body(buffer)
    } else {
        HttpResponse::NoContent().finish()
    }
}