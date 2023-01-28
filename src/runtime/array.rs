pub trait Storable: Sync {}
pub struct Array {
  atype: ArrayType,
  len: u32,
  data: Vec<Box<dyn Storable>>
}

impl Array {
  pub fn prim_new<T: Storable + Default + 'static>(len: u32) -> Self {
    let mut data: Vec<Box<dyn Storable>> = Vec::new();
    for _ in 0..len {
      data.push(Box::new(T::default()));
    }
    
    Self {
      atype: ArrayType::Prim,
      len,
      data
    }
  }
}

enum ArrayType {
  Prim,
  Ref
}
macro_rules! define {
  ($t: ty) => {
    impl Storable for $t {}
  }
}

define!(u8);
define!(i8);
define!(i16);
define!(i32);
define!(i64);
define!(char);