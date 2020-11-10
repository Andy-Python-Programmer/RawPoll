use rocket::*;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/")]
pub fn get() -> Option<NamedFile> {
    return NamedFile::open(Path::new("public/").join("index.html")).ok();
}