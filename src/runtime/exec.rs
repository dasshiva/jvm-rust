use crate::classfile::mets::MetInfo;
use crate::classfile::cpool::CPool;
use crate::classfile::attrs::*;

pub fn exec(method: &MetInfo, pool: &CPool) {
  let find = |&method| -> Code {
    for attr in method.attrs {
      match attr {
        Attrs::CodeAttr(s) => return s,
        _ => {}
      }
    }
    
    panic!("No Code attribute found")
  }
  
  let code = find(method);
  
}