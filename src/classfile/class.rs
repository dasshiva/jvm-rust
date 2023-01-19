use std::fs;
use std::io::*;
use crate::classfile::cpool::CPool;
use crate::classfile::mets::Methods;

pub struct ClassFile {
  _min_ver: u16,
  _max_ver: u16,
  pub cpool: CPool,
  _flags : u16,
  this_class: u16,
  super_class: u16,
  _inters_count: u16,
  //_inters: u16
  _flds_count: u16,
  pub mets: Methods
}

pub(crate) fn read_u4(src: &mut Cursor<Vec<u8>>) -> u32 {
  let mut buf = [0u8; 4];
  src.read(&mut buf);
  u32::from_be_bytes(buf)
}

pub(crate) fn read_u2(src: &mut Cursor<Vec<u8>>) -> u16 {
  let mut buf = [0u8; 2];
  src.read(&mut buf);
  u16::from_be_bytes(buf)
}

pub(crate) fn read_u1(src: &mut Cursor<Vec<u8>>) -> u8 {
  let mut buf = [0u8; 1];
  src.read(&mut buf);
  buf[0]
}

impl ClassFile {
  pub fn new(file: &str) -> Self {
    let buf = match fs::read(file) {
      Ok(res) => res,
      Err(..) => panic!("File {file} not found")
    };
    let mut cursor = Cursor::new(buf);
    if read_u4(&mut cursor) != 0xCAFEBABE {
      panic!("Invalid file magic")
    }
   
    let _min_ver = read_u2(&mut cursor);
    let _max_ver = read_u2(&mut cursor);
    let cpool = CPool::new(&mut cursor);
    let  _flags = read_u2(&mut cursor);
    let this_class = read_u2(&mut cursor);
    let super_class = read_u2(&mut cursor);
    let _inters_count = read_u2(&mut cursor);
    let _flds_count = read_u2(&mut cursor);
    let mets = Methods::new(&mut cursor, &cpool);
    
    ClassFile {
      _min_ver,
      _max_ver,
      cpool,
      _flags,
      this_class,
      super_class,
      _inters_count,
      _flds_count,
      mets
    }
  }
}