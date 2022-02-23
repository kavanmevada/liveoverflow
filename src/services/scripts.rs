use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::path::PathBuf;

#[actix_web::get("/scripts/{filename:.*}")]
pub async fn get_scripts(req: HttpRequest) -> Option<NamedFile> {
    req.match_info()
        .query("filename")
        .parse::<PathBuf>()
        .ok()
        .and_then(|path| NamedFile::open(PathBuf::new().join("scripts").join(path)).ok())
}
