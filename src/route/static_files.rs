use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/images/<image..>")]
pub fn images(image: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("images/").join(image)).ok()
}
