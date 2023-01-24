use crate::classfile::attrs::Attrs;
use crate::classfile::attrs::*;
use crate::classfile::cpool::CPool;
use crate::runtime::frame::*;
use crate::runtime::class::Method;

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
                let elem = frame.lvarray.remove(delta as usize);
                match elem {
                    Types::Int(..) => {}
                    _ => panic!("iload_<{delta}> used but argument is not an int"),
                }
                frame.stack.push(elem);
            }
            59 | 60 | 61 | 62 => {
                // istore_<n>
                let delta = s.code[pc as usize] - 59u8;
                let elem = frame.stack.pop();
                match elem {
                    Types::Int(..) => {}
                    _ => panic!("istore_<{delta}> used but argument is not a in int"),
                }
                if frame.lvarray.len() <= delta as usize {
                    frame.lvarray.push(elem);
                } else {
                    frame.lvarray[delta as usize] = elem;
                }
            }
            96 => {
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
            177 => break,
            _ => panic!("Unimplemented instruction {}", s.code[pc as usize]),
        }
        pc += 1;
    }
}
