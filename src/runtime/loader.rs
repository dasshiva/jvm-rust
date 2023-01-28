use lazy_static::lazy_static; 
use owning_ref::MutexGuardRef;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::classfile::class::ClassFile;
use crate::runtime::class::Class;
use crate::runtime::heap::*;

lazy_static! {
  static ref LOADED: Mutex<Vec<&'static HeapRef>> = Mutex::new(Vec::new());
  static ref COUNT: AtomicUsize = AtomicUsize::new(0);
}

fn load(file: &str) -> Result<(), ()>{
  let cf = match ClassFile::new(file) {
    Some(c) => c,
    None => return Err(())
  };
  LOADED.lock().unwrap().push(new_class(Class::new(cf)));
  COUNT.fetch_add(1, Ordering::SeqCst);
  Ok(())
}

fn get_latest_class() -> &'static Class {//MutexGuardRef<'static, Vec<Class>, Class> {
  let guard = MutexGuardRef::new(LOADED.lock().unwrap()).map(|mg| mg.get(COUNT.load(Ordering::Relaxed) - 1).unwrap());
  match *guard {
    HeapRef::Object(s) => s,
    _ => unreachable!()
  }
}

pub fn get_class_by_name(name: &str) -> Option<&'static Class> {//-> Option<MutexGuardRef<'static, Vec<Class>, Class>> {
  let mut i = 0usize;
  for i in 0..LOADED.lock().unwrap().len() {
    match LOADED.lock().unwrap()[i] {
      HeapRef::Object(s) => {
        if &s.name == &name[0..name.find(".").unwrap()] {
          return Some(s);
        }
      }
      _ => {}
    }
    i += 1;
  }
  match load(name) {
    Ok(..) => Some(get_latest_class()),
    Err(..) => None
  }
}