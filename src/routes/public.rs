use rocket::*;
use rocket::response::NamedFile;
use std::path::{PathBuf, Path};

#[get("/static/<file..>")]
pub fn get(file: PathBuf) -> Option<NamedFile> {
    let path: PathBuf = Path::new("static").join(file);

    return NamedFile::open(path).ok();
}