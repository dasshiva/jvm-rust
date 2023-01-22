use std::io::*;
use crate::classfile::class::*;
use crate::classfile::cpool::*;

pub enum Attrs {
  CodeAttr(Code),
  Source(String)
}

impl Attrs {
  pub fn mt_attrs(src: &mut Cursor<Vec<u8>>, pool: &CPool) -> Vec<Attrs> {
    let size = read_u2(src);
    let mut attrs: Vec<Attrs> = Vec::with_capacity(size as usize);
    for i in 0..size {
      let idx = pool.get_utf8(read_u2(src));
      match idx.as_str() {
        "Code" => attrs.push(Attrs::CodeAttr(Code::new(src, pool))),
        _ => panic!("Unrecognised method attribute")
      }
    }
    attrs
  }
  
  pub fn cl_attrs(src: &mut Cursor<Vec<u8>>, pool: &CPool) -> Vec<Attrs> {
    let size = read_u2(src);
    let mut attrs: Vec<Attrs> = Vec::with_capacity(size as usize);
    for i in 0..size {
      let idx = pool.get_utf8(read_u2(src));
      read_u4(src);
      match idx.as_str() {
        "SourceFile" => attrs.push(Attrs::Source(pool.get_utf8(read_u2(src)))),
        _ => panic!("Unrecognised class attribute")
      }
    }
    attrs
  }
}

// TODO: Implement exception table
pub struct Code {
  pub stack: u16,
  pub locals: u16,
  pub code: Vec<u8>,
  _exc_tab: u16,
  _attrs_count: u16
}

impl Code {
  pub fn new(src: &mut Cursor<Vec<u8>>, pool: &CPool) -> Self {
    read_u4(src);
    let stack = read_u2(src);
    let locals = read_u2(src);
    let len = read_u4(src);
    let mut code: Vec<u8> = Vec::new();
    for i in 0..len {
      code.push(read_u1(src));
    }
    
    let _exc_tab = match read_u2(src) {
      0 => 0,
      _ => panic!("Exceptions are not supported")
    };
    
    let _attrs_count = match read_u2(src) {
      0 => 0,
      _ => {
        read_u2(src);
        let skip = read_u4(src);
        src.consume(skip as usize);
        0
      }
    };
    
    Code {
      stack,
      locals,
      code,
      _exc_tab,
      _attrs_count
    }
  }
}