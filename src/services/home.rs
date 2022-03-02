use actix_web::HttpResponse;
use actix_web::Responder;
use std::io::Read;

#[actix_web::get("/home")]
pub async fn get_index() -> impl Responder {
    if let Some(render) = std::fs::File::open("static/home.html")
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
        HttpResponse::Ok().body(render.replace("/* data-username */", "{ apple: \"string\" }"))
    } else {
        HttpResponse::NoContent().body("")
    }
}
