use std::io::*;
use crate::classfile::class::*;
use crate::classfile::cpool::CPool;

pub struct Methods {
  methods: Vec<MetInfo>,
  size: u16
}

impl Methods {
  pub fn new(src: &mut Cursor<Vec<u8>>) -> Self {
    let size = read_u2(src);
    let mut methods: Vec<MetInfo> = Vec::new();
    for i in 0..size {
      methods.push(MetInfo::new(src))
    }
    
    Methods {
      size,
      methods
    }
  }
}

pub struct MetInfo {
  _flags: u16,
  name: u16,
  desc: u16,
  attrs_count: u16
}

impl MetInfo {
  pub fn new(src: &mut Cursor<Vec<u8>>) -> Self {
    MetInfo {
      _flags: read_u2(src),
      name: read_u2(src),
      desc: read_u2(src),
      attrs_count: read_u2(src)
    }
  }
}