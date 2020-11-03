use rocket::*;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/new")]
pub fn get() -> Option<NamedFile> {
    return NamedFile::open(Path::new("public/").join("create.html")).ok();
}