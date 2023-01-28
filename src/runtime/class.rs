use crate::classfile::*;

pub struct Method {
  pub name: String,
  pub desc: String,
  pub flags: MethodFlags,
  pub code: attrs::Code,
}

impl Method {
  pub fn new(met: mets::MetInfo) -> Self {
    Self {
      name: met.name,
      desc: met.desc,
      flags: MethodFlags::new(met.flags),
      code: {
        match &met.attrs[0] {
          attrs::Attrs::CodeAttr(s) => s.clone(),
          _ => panic!("No Code attribute found")
        }
      }
    }
  }
}

pub struct MethodFlags {
  pub public: bool,
  pub private: bool,
  pub prot: bool,
  pub stat: bool,
  pub native: bool,
  pub abs: bool
}

impl MethodFlags {
  pub fn new(flags: u16) -> Self {
    Self {
      public: (flags & 0x0001) != 0,
      private: (flags & 0x0002) != 0,
      prot: (flags & 0x0004) != 0,
      stat: (flags & 0x0008) != 0,
      native: (flags & 0x100) != 0,
      abs: (flags & 0x0400) != 0
    }
  }
}

pub struct Class {
  pub name: String,
  pub parent: String,
  pub cpool: cpool::CPool,
  pub vtable: Vec<Method>,
  //attrs: Vec<attrs::Attrs>
}

impl Class {
  pub fn new(mut cf: class::ClassFile) -> Self {
    let name = cf.cpool.get_inner_utf8(cf.this_class);
    let parent = cf.cpool.get_inner_utf8(cf.super_class);
    let mut vtable: Vec<Method> = Vec::new();
    let mut i = 0usize;
    while i < cf.mets.methods.len()+1 {
      if i != 0 {
        vtable.push(Method::new(cf.mets.methods.remove(i - 1)));
      }
      else {
        vtable.push(Method::new(cf.mets.methods.remove(i)));
      }
      i += 1;
    }
    //let attrs = cf.attrs;
    Self {
      name,
      parent,
      cpool: cf.cpool,
      vtable,
      //attrs
    }
  }
  
  pub fn find(&self, name: &str, desc: &str) -> &Method {
    for i in 0..self.vtable.len() {
      if self.vtable[i].name == name {
        if self.vtable[i].desc == desc {
          return &self.vtable[i];
        }
      }
    }
    
    panic!("Method {name} with signature {desc} not found ")
  }
}