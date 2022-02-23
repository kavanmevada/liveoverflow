use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};
use std::io::Read;

use crate::models::{AppContext, Person, Session};

#[actix_web::get("/account")]
async fn get_account(data: Data<AppContext>, req: HttpRequest) -> impl Responder {
    let mut person = None;

    for query in req.query_string().split('&') {
        if let Some((_, uuid)) = query.split_once('=') {
            let mut stmt = data
                .connection
                .prepare("select * from sessions where session=:session;")
                .expect("Error selecting row");

            let mut session_iter = stmt
                .query_map(&[(":session", uuid)], |row| {
                    Ok(Session {
                        _session: row.get(1)?,
                        username: row.get(2)?,
                    })
                })
                .expect("Error getting rows");

            if let Some(Ok(session)) = session_iter.next() {
                let mut stmt = data
                    .connection
                    .prepare("select * from person where username=:username;")
                    .expect("Error selecting row");

                let mut person_iter = stmt
                    .query_map(&[(":username", &session.username)], |row| {
                        Ok(Person {
                            // id: row.get(0)?,
                            username: row.get(1)?,
                            email: row.get(2)?,
                            password: row.get(3)?,
                        })
                    })
                    .expect("Error getting rows");

                if let Some(Ok(p)) = person_iter.next() {
                    person = Some(p)
                }
            }
        }
    }

    if let (Some(render), Some(person)) = (
        std::fs::File::open("static/index.html")
            .ok()
            .and_then(|mut f| {
                let mut buf = Vec::new();
                if f.read_to_end(&mut buf).is_ok() {
                    String::from_utf8(buf).ok()
                } else {
                    None
                }
            }),
        person, /* person.and_then(|p| serde_json::to_string(&p).ok()) */
    ) {
        HttpResponse::Ok().body(render.replace("/* data-username */", &person.username))
    } else {
        HttpResponse::Found().header("Location", "/").finish()
    }
}
