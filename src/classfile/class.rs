use std::fs;

pub struct ClassFile;

impl ClassFile {
  pub fn new(file: &str) {
    let buf = match fs::read(file) {
      Ok(res) => res,
      Err(..) => panic!("File {file} not found")
    };
  }
}