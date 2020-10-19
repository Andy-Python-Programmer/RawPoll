use rocket::response::NamedFile;
use std::path::{ Path, PathBuf };

pub type Page = Option<NamedFile>;
pub type File = PathBuf;

pub struct Templating {}

#[derive(serde::Serialize)]
pub struct PollTemplate {
    pub title: String,
    pub description: String,
    pub options: String
}

#[derive(serde::Serialize)]
pub struct PollNew {
    pub id: String
}

impl Templating {
    pub fn new() -> Templating {
        return Templating {};
    }

    pub fn render_static(&self, file: File) -> Option<NamedFile> {
        let path: PathBuf = Path::new("static").join(file);

        return NamedFile::open(path).ok();
    }

    pub fn render(&self, file_name: &str) -> Option<NamedFile> {
        return NamedFile::open(Path::new("public/").join(file_name)).ok();
    }
}
