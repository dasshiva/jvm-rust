use lazy_static::lazy_static; 
use owning_ref::MutexGuardRef;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::{heap, class::Class};
use crate::classfile::class::ClassFile;
use crate::runtime::heap::*;

pub fn get_class_by_name<'a>(file: &str, heap: &'a mut Heap) -> Option<&'a Class> {
  for i in &heap.data {
    match i {
      HeapRef::Object(s) => return Some(s),
      _ => {}
    }
  }
  
  Some(heap.new_class(Class::new(mut ClassFile::new(file))).as_obj().unwrap())
}