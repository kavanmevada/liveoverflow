use actix_web::{
    web::{Data, Form},
    HttpRequest, HttpResponse, Responder,
};
use rusqlite::params;
use std::io::Read;

use crate::models::{AppContext, Login, Person};

#[actix_web::post("/login")]
async fn post_login(
    data: Data<AppContext>,
    form: Form<Login>,
    _req: HttpRequest,
) -> impl Responder {
    let mut stmt = data
        .connection
        .prepare("select * from person where username=:username and password=:password;")
        .expect("Error selecting row");

    let mut person_iter = stmt
        .query_map(
            &[(":username", &form.username), (":password", &form.password)],
            |row| {
                Ok(Person {
                    // id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    password: row.get(3)?,
                })
            },
        )
        .expect("Error getting rows");

    if let Some(Ok(person)) = person_iter.next() {
        let sha256 = openssl::sha::sha256(person.username.as_bytes());
        let sha256_str = sha256
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        data.connection
            .execute(
                "insert into sessions (session, username) values (?1, ?2)",
                params![sha256_str, person.username],
            )
            .expect("msg");

        HttpResponse::Found()
            .header("Location", format!("/account?uuid={}", sha256_str))
            .finish()
    } else if let Some(render) = std::fs::File::open("static/login.html")
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

#[actix_web::get("/login")]
async fn get_login(_req: HttpRequest) -> impl Responder {
    if let Some(render) = std::fs::File::open("static/login.html")
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
