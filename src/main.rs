mod classfile;
mod runtime;
mod logger;
use classfile::class::ClassFile;
use runtime::exec;
use runtime::heap::Heap;
use std::panic;
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate owning_ref;
use runtime::loader;

fn main() {
  logger::init();
  let mut heap = Heap::init();
  panic::set_hook(Box::new(|panic_info| {
    if let Some(s) = panic_info.payload().downcast_ref::<String>(){
      log::error!("{s}");
    } 
    else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
      log::error!("{s}");
    }
  }));
  
  let class = match loader::get_class_by_name("Hello.class", &mut heap) {
    Some(s) => s,
    None => panic!("Main class not found")
  };
  let main = class.find("add", "()V");
  exec::run(&main, &class.cpool, &mut heap);
}
