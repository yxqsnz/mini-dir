use std::fs::{self, DirEntry};
use std::format;
pub fn recursive(path: String) {
    print!("Reading '{}'\n\n",path);
    let paths = fs::read_dir(path);

    for path in paths.unwrap(){
        let entry = path.unwrap();
        println!("{}", format!("+ {} {:#?}",fetch_type(&entry), entry.file_name()));
        if entry.path().is_dir() {
            recursive(entry.path().display().to_string())
        }
    }
}
fn fetch_type(entry: &DirEntry) -> String {
    if entry.path().is_dir() {
       String::from("<DIR>")
    } else {
       String::from("<FILE>")
    }
}