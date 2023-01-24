mod classfile;
mod runtime;
mod logger;
use classfile::class::ClassFile;
use runtime::exec;
use runtime::class::Class;
use std::panic;
extern crate log;

fn main() {
  logger::init();
  panic::set_hook(Box::new(|panic_info| {
    if let Some(s) = panic_info.payload().downcast_ref::<String>(){
      log::error!("{s}");
    } 
    else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      log::error!("{s}");
    }
  }));
  
  let class = Class::new(ClassFile::new("Hello.class"));
  let main = class.find("add", "()V");
  exec::run(&main, &class.cpool);
}
