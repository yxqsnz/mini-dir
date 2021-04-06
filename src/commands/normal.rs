use std::fs::{self, DirEntry};
use std;
use std::format;
pub fn show_files(path: String) {
  print!("Scanning for entrys in '{}' ... ",String::clone(&path));
  let files = fs::read_dir(String::clone(&path));
  
  let mut files_in_path = Vec::new();
  for file in files.unwrap() {
    let unw = file.unwrap();
    let typ = fetch_type(&unw);
    files_in_path.push(format!("{} {:?}",typ,unw.file_name()))
  }

  println!("Found {:#?} entrys in '{}'",files_in_path.len(),String::clone(&path));
  for file in files_in_path{
    println!("+ {}", file);
  }
}
fn fetch_type(entry: &DirEntry) -> String{
  if entry.path().is_dir() {
     String::from("<DIR>")
  } else {
     String::from("<FILE>")
  }
}