pub struct Frame {
  stack: Stack,
  lvarray: Vec<Types>
}

struct Stack {
  size: u16,
  top: u16,
  buf: Vec<Types>
}

enum Types {
  Int(i32),
  Long(i64),
  Float(f32),
  Double(f64)
}