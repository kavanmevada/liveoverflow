use actix_web::{web::Data, HttpRequest, HttpResponse, Responder, HttpMessage};
use std::{io::Read, fs::File};

use crate::{models::{AppContext}, sqlite3::{self, LoginType}};

#[actix_web::get("/profile")]
async fn get_profile(data: Data<AppContext>, req: HttpRequest) -> impl Responder {
    let mut builder = HttpResponse::Found();

    let mut buffer = String::new();
    let page = File::open("static/profile.html").and_then(|mut f| f.read_to_string(&mut buffer));
    if let Some(sessionid) = req.cookie("__Secure-id") {
        let ret = sqlite3::login(
            &data.connection,
            LoginType::Session(&sessionid.value()),
        );

        if page.is_ok() {
            dbg!(ret);

            builder
                .body(buffer)
        } else {
            HttpResponse::NotFound().finish()
        }
    } else {
        builder
            .header("Location", "/login")
            .finish()
    }
}
