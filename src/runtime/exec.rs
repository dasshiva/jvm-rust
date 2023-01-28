use crate::classfile::attrs::Attrs;
use crate::classfile::attrs::*;
use crate::classfile::cpool::CPool;
use crate::runtime::frame::*;
use crate::runtime::class::Method;
use crate::runtime::r#ref::Reference;
use crate::runtime::array::Array;

pub fn run(method: &Method, pool: &CPool) {
    log::info!(
        "Starting method {} with signature {}",
        method.name,
        method.desc
    );
    real_exec(&method.code, &mut Frame::new(method));
    log::info!("Method finished execution successfully");
}

pub fn real_exec(s: &Code, frame: &mut Frame) {
    let mut pc = 0u16;
    while pc < s.code.len() as u16 {
        match s.code[pc as usize] {
            2 | 3 | 4 | 5 | 6 | 7 | 8 => {
              // iconst_<n>
              let elem = s.code[pc as usize] - 3u8;
              frame.stack.push(Types::Int(elem as i32));
            }
            16 => {
                // bipush
                frame
                    .stack
                    .push(Types::Int(s.code[(pc + 1) as usize] as i32));
                pc += 1;
            }
            17 => {
                // sipush
                let byte1 = s.code[(pc + 1) as usize] as u16;
                let byte2 = s.code[(pc + 2) as usize] as u16;
                frame
                    .stack
                    .push(Types::Int(((byte1 << 8u8) | byte2) as i32));
                pc += 2;
            }
            26 | 27 | 28 | 29 => {
                // iload_<n>
                let delta = s.code[pc as usize] - 26u8;
                if delta as usize >= frame.lvarray.len() {
                    panic!("iload_<{delta}> used with invalid index into local variable array")
                }
                let elem = frame.lvarray[delta as usize];
                match elem {
                  Types::Int(..) => {}
                   _ => panic!("iload_<{delta}> used but argument is not a reference"),
                }
                frame.stack.push(elem);
            }
            42 | 43 | 44 | 45 => {
              // aload_<n>
              let delta = s.code[pc as usize] - 42u8;
              if delta as usize >= frame.lvarray.len() {
                  panic!("aload_<{delta}> used with invalid index into local variable array")
              }
              let elem = &frame.lvarray[delta as usize];
              let topush = match elem {
                Types::Ref(s) => s,
                 _ => panic!("aload_<{delta}> used but argument is not a reference"),
              };
                frame.stack.push(Types::Ref(topush));
            }
            59 | 60 | 61 | 62 => {
                // istore_<n>
                let delta = s.code[pc as usize] - 59u8;
                let elem = frame.stack.pop();
                match elem {
                    Types::Int(..) => {}
                    _ => panic!("istore_<{delta}> used but argument is not an int"),
                }
               frame.lvarray[delta as usize] = elem;
            }
            75 | 76 | 77 | 78 => {
              // astore_<n>
               let delta = s.code[pc as usize] - 75u8;
               let elem = frame.stack.pop();
               match elem {
                  Types::Ref(..) => {}
                   _ => panic!("astore_<{delta}> used but argument is not a reference"),
               }
               frame.lvarray[delta as usize] = elem;
            }
            96 => {
              // iadd
                let add1 = match frame.stack.pop() {
                    Types::Int(i) => i,
                    _ => panic!("iadd used but argument is not an int"),
                };

                let add2 = match frame.stack.pop() {
                    Types::Int(i) => i,
                    _ => panic!("iadd used but argument is not an int"),
                };
                frame.stack.push(Types::Int(add1 + add2));
            }
            132 => {
              // iinc
              let index = s.code[(pc + 1) as usize] as usize;
              let inc = s.code[(pc + 2) as usize] as i32;
              match frame.lvarray[index] {
                Types::Int(s) => frame.lvarray[index] = Types::Int(s + inc),
                Types::Empty => panic!("Empty"),
                _ => panic!("iinc used but argument is not int")
              } 
              pc += 2;
            }
            159..=164 => {
              // ifcmp_<cond>
              let e1 = match frame.stack.pop() {
                Types::Int(i) => i,
                _ => panic!("ifcmp used but argument is not an int"),
              };
              
              let e2 = match frame.stack.pop() {
                Types::Int(i) => i,
                 _ => panic!("ifcmp used but argument is not an int"),
              };
              let mut res = false;
              match s.code[pc as usize] {
                159 => res = e1 == e2,
                160 => res = e1 != e2,
                161 => res = e1 < e2,
                162 => res = e1 <= e2,
                163 => res = e1 > e2,
                164 => res = e1 >= e2,
                _ => panic!("Invalid instruction")
              }
              if !res {
                pc += 2;
              }
              else {
                let byte1 = s.code[(pc + 1) as usize] as u16;
                let byte2 = s.code[(pc + 2) as usize] as u16;
                let offset = (byte1 << 8u8) | byte2;
                pc += offset;
                continue;
              }
            }
            177 => break,// return
            188 => {
              // newarray
              pc += 1;
              let atype = s.code[pc as usize];
              let size = match frame.stack.pop() {
                Types::Int(i) => i,
                 _ => panic!("newarray used but size is not an int")
              };
              match atype {
                4 => frame.stack.push(Types::Ref(Reference::new_arr(Array::prim_new::<u8>(size as u32)))),
                5 => frame.stack.push(Types::Ref(Reference::new_arr(Array::prim_new::<char>(size as u32)))),
                _ => panic!("Unknown array type")
              }
            }
            _ => panic!("Unimplemented instruction {}", s.code[pc as usize]),
        }
        pc += 1;
    }
}
