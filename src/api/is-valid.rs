use actix_web::{web::{Data, Form}, HttpRequest, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

use crate::{models::AppContext, sqlite3};

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData {
    pub query: String,
}

#[actix_web::post("/api/validate/username")]
async fn post_is_valid_username(data: Data<AppContext>, form: Form<FormData>, _req: HttpRequest) -> impl Responder {
    let person = sqlite3::person(&data.connection,  ("username", &form.query));
    let mut ret = HttpResponse::Found();
    if person.is_some() {
        ret.body(r#"{ "result":"error", "type":"username", "msg":"User Name invalid" }"#)
    } else {
        ret.body(r#"{ "result":"ok", "error": null }"#)
    }
}

#[actix_web::post("/api/validate/email")]
async fn post_is_valid_email(data: Data<AppContext>, form: Form<FormData>, _req: HttpRequest) -> impl Responder {
    let person = sqlite3::person(&data.connection,  ("email", &form.query));
    let mut ret = HttpResponse::Found();
    if person.is_some() {
        ret.body(r#"{ "result":"error", "type":"email", "msg":"Email is already registered!" }"#)
    } else if !form.query.contains("@") {
        ret.body(r#"{ "result":"error", "type":"email", "msg":"Email is invalid!" }"#)   
    } else {
        ret.body(r#"{ "result":"ok", "error": null }"#)
    }
}
