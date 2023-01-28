use crate::runtime::array::Array;
use crate::runtime::class::Class;

pub struct Heap {
    pub data: Vec<HeapRef>,
}
pub enum HeapRef {
    Object(Class),
    Array(Array),
}

impl HeapRef {
  pub fn as_obj(&mut self) -> Option<&mut Class> {
    match self {
      HeapRef::Object(s) => Some(s),
      _ => None
    } 
  }
}
impl Heap {
    pub fn init() -> Self {
        Self { data: Vec::new() }
    }

    pub fn new_class<'a>(&'a mut self, class: Class) -> &'a mut HeapRef {
        self.data.push(HeapRef::Object(class));
        let len = self.data.len();
        &mut self.data[len - 1]
    }

    pub fn new_array<'a>(&'a mut self, array: Array) -> &'a mut HeapRef {
        self.data.push(HeapRef::Array(array));
        let len = self.data.len();
        &mut self.data[len - 1]
    }
}
