mod classfile;
mod logger;
use classfile::class::ClassFile;
use std::panic;
extern crate log;

fn main() {
  logger::init();
  panic::set_hook(Box::new(|panic_info| {
    if let Some(s) = panic_info.payload().downcast_ref::<String>(){
      println!("{s}");
    } 
    else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      println!("panic ooccurred: {s:?}")
    }
    else {
      println!("panic occurred");
    }
  }));
  
  let file = ClassFile::new("Hello.class");
  let main = file.mets.find("add", "(II)V");
}
