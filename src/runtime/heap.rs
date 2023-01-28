use lazy_static::lazy_static;
use crate::runtime::array::Array;
use crate::runtime::class::Class;

lazy_static! {
  static ref HEAP: Vec<HeapRef> = Vec::new();
}

pub enum HeapRef {
  Object(Class),
  Array(Array)
}

pub fn new_class(class: Class) -> &'static HeapRef {
  HEAP.push(HeapRef::Object(class));
  &HEAP[HEAP.len() - 1]
}

pub fn new_array(array: Array) -> &'static HeapRef {
  HEAP.push(HeapRef::Array(array));
  &HEAP[HEAP.len() - 1]
}