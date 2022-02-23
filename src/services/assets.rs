use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::path::PathBuf;

#[actix_web::get("/assets/{filename:.*}")]
pub async fn get_assets(req: HttpRequest) -> Option<NamedFile> {
    req.match_info()
        .query("filename")
        .parse::<PathBuf>()
        .ok()
        .and_then(|path| NamedFile::open(PathBuf::new().join("assets").join(path)).ok())
}
