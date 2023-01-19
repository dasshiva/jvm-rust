use std::io::*;
use crate::classfile::class::*;
use crate::classfile::cpool::CPool;
use crate::classfile::attrs::Attrs;
pub struct Methods {
  methods: Vec<MetInfo>,
  size: u16
}

impl Methods {
  pub fn new(src: &mut Cursor<Vec<u8>>, pool: &CPool) -> Self {
    let size = read_u2(src);
    let mut methods: Vec<MetInfo> = Vec::new();
    for i in 0..size {
      methods.push(MetInfo::new(src, pool));
    }
    
    Methods {
      size,
      methods
    }
  }
}

pub struct MetInfo {
  _flags: u16,
  name: String,
  desc: String,
  attrs: Vec<Attrs>
}

impl MetInfo {
  pub fn new(src: &mut Cursor<Vec<u8>>, pool: &CPool) -> Self {
    MetInfo {
      _flags: read_u2(src),
      name: pool.get_utf8(read_u2(src)),
      desc: pool.get_utf8(read_u2(src)),
      attrs: Attrs::mt_attrs(src, pool)
    }
  }
}