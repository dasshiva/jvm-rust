use crate::runtime::class::*;
use crate::runtime::array::*;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use owning_ref::OwningRef;

pub struct Reference {
    id: u64,
    reftype: RefType,
    data: Option<&'static Class>,
    array: Option<Array>
}

impl Reference {
  pub fn new_obj(class: &'static Class) -> Reference {
    Self {
      id: SmallRng::from_entropy().next_u64(),
      reftype: RefType::Object,
      data: Some(class),
      array: None::<Array>
    }
  }
  
  pub fn new_arr(array: Array) -> Reference {
    Self {
      id: SmallRng::from_entropy().next_u64(),
      reftype: RefType::Array,
      data: None::<&Class>,
      array: Some(array)
    }
  }
  
  pub fn new_null() -> Reference {
    Self {
      id: SmallRng::from_entropy().next_u64(),
      reftype: RefType::Null,
      data: None::<&Class>,
      array: None::<Array>
    }
  }
}

enum RefType {
    Object,
    Array,
    Null
}
