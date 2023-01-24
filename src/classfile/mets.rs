use std::io::*;
use crate::classfile::class::*;
use crate::classfile::cpool::CPool;
use crate::classfile::attrs::Attrs;
use crate::classfile::attrs::Code;
pub struct Methods {
  pub methods: Vec<MetInfo>,
  pub size: u16
}

impl Methods {
  pub fn new(src: &mut Cursor<Vec<u8>>, pool: &CPool) -> Self {
    let size = read_u2(src);
    let mut methods: Vec<MetInfo> = Vec::new();
    for i in 0..size {
      methods.push(MetInfo::new(src, pool));
      log::info!("Found method {} with signature {}", methods[i as usize].name, methods[i as usize].desc);
    }
    
    Methods {
      size,
      methods
    }
  }
}

pub struct MetInfo {
  pub flags: u16,
  pub name: String,
  pub desc: String,
  pub attrs: Vec<Attrs>
}

impl MetInfo {
  pub fn new(src: &mut Cursor<Vec<u8>>, pool: &CPool) -> Self {
    MetInfo {
      flags: read_u2(src),
      name: pool.get_utf8(read_u2(src)),
      desc: pool.get_utf8(read_u2(src)),
      attrs: Attrs::mt_attrs(src, pool)
    }
  }
}