use rocket::*;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/vote/<_id>")]
pub fn get(_id: String) -> Option<NamedFile> {
    return NamedFile::open(Path::new("public/").join("vote.html")).ok();
}