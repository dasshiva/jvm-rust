use crate::classfile::mets::MetInfo;
use crate::classfile::cpool::CPool;
use crate::classfile::attrs::Attrs;
use crate::classfile::attrs::*;
use crate::runtime::frame::*;

pub fn run(method: &MetInfo, pool: &CPool) { 
 log::info!("Starting method {} with signature {}", method.name, method.desc);
 for attr in &method.attrs {
    match attr {
      Attrs::CodeAttr(s) => {
        let mut frame = Frame::new(&method.desc, s.stack, s.locals);
        let mut pc = 0u16;
        while pc < s.code.len() as u16 {
          match s.code[pc as usize] {
            17 => { // sipush
              let byte1 = s.code[(pc + 1) as usize] as u16;
              let byte2 = s.code[(pc + 2) as usize] as u16;
              frame.stack.push(Types::Int(((byte1 << 8u8) | byte2) as i32));
              pc += 2;
            }
            59 | 60 | 61 | 62 => { // istore_<n>
              let delta = s.code[pc as usize] - 59u8;
              let elem = frame.stack.pop();
              if frame.lvarray.len() <= delta as usize {
                frame.lvarray.push(elem);
              }
              else {
                frame.lvarray[delta as usize] = elem;
              }
            }
            
            177 => break,
            _ => panic!("Unimplemented instruction {}", s.code[pc as usize])
          }
          pc += 1;
        }
        break;
      }
      _ => {}
    }
 }
 
 log::info!("Method finished execution successfully");
}