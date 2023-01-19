use std::io::*;
use crate::classfile::class::{read_u1, read_u2};

pub struct CPool {
  size: u16,
  pool: Vec<Elem>
}

impl CPool {
  pub fn new(src: &mut Cursor<Vec<u8>>) -> Self {
    let size = read_u2(src);
    let mut pool: Vec<Elem> = Vec::with_capacity((size - 1) as usize);
    for i in 0..size-1 {
      let tag = read_u1(src);
      match tag {
        1 => {
          let mut buf = String::new();
          let length = read_u2(src);
          for i in 0..length {
            buf.push(read_u1(src) as char);
          }
          pool.push(Elem::Utf8(buf));
        },
        7 => pool.push(Elem::Class(read_u2(src))),
        8 => pool.push(Elem::Str(read_u2(src))),
        16 => pool.push(Elem::MType(read_u2(src))),
        10 => {
          let p1 = read_u2(src);
          let p2 = read_u2(src);
          pool.push(Elem::Method(p1, p2));
        },
        12 => {
          let p1 = read_u2(src);
          let p2 = read_u2(src);
          pool.push(Elem::Name(p1, p2));
        },
        _ => panic!("Unknown tag {tag}")
      }
    }
    
    CPool {
      size,
      pool
    }
  }
  
  pub fn get(&self, index: u16) -> &Elem {
    if index < 1 || index > self.size {
      panic!("Invalid constant pool index {index}")
    }
    &self.pool[(index - 1) as usize]
  }
  
  pub fn get_utf8(&self, index: u16) -> String {
    match self.get(index) {
      Elem::Utf8(s) => s.clone(),
      _ => panic!("Index {index} is not a CONSTANT_Utf8 element")
    }
  }
}


pub enum Elem {
  Utf8(String), // CONSTANT_Utf8
  Class(u16),   // CONSTANT_Class
  Str(u16),     // CONSTANT_String
  Method(u16, u16), // CONSTANT_MethodRef
  Name(u16, u16), // CONSTANT_NameAndType
  MType(u16)    // CONSTANT_MethodType
}