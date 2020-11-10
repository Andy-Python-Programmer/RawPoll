use rocket::*;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/poll/<_id>")]
pub fn get(_id: String) -> Option<NamedFile> {
    return NamedFile::open(Path::new("public/").join("poll.html")).ok();
}