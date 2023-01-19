mod classfile;
use classfile::class::ClassFile;
use std::panic;

fn main() {
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
  let main = file.mets.find("main", "([Ljava/lang/String;)V");
}
