use rocket::*;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;
use std::path::{PathBuf, Path};

#[get("/static/<file..>")]
pub fn get(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    // Do not send the typescript files
    if &file.to_str().unwrap().contains("typescript") == &true{

        // Send Access Denied if they are trying to access typescript files.
        return Err(NotFound("Access Denied".to_string()));
    }

    let path: PathBuf = Path::new("static").join(file);

    // If everything is cool and they are trying to access the right files then send them the file!
    return Ok(NamedFile::open(path).unwrap());
}