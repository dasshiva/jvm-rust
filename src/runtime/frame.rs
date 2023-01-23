pub struct Frame {
  pub stack: Stack,
  pub lvarray: Vec<Types>
}

impl Frame {
  pub fn new(desc: &str, stack: u16, locals: u16) -> Self {
    let end = desc.chars().position(|c| c == ')').unwrap();
    let args = &desc[1..end];
    let ret = &desc[end+1..desc.len()];
    let mut lvarray: Vec<Types> = Vec::with_capacity(locals as usize);
    for i in args.chars() {
      match i {
        'I' => lvarray.push(Types::Int(0)),
        'J' => lvarray.push(Types::Long(0)),
        'D' => lvarray.push(Types::Double(0.0)),
        'F' => lvarray.push(Types::Float(0.0)),
         _ => unreachable!()
      }
    }
    
    Self {
      stack: Stack::new(stack),
      lvarray
    }
  }
}

pub struct Stack {
  size: u16,
  top: u16,
  buf: Vec<Types>
}

impl Stack {
  pub fn new(size: u16) -> Self {
    Self {
      size,
      top: 0,
      buf: Vec::with_capacity(size as usize)
    }
  }
  
  pub fn push(&mut self, elem: Types) {
    if self.top == self.size {
      panic!("Stack overflow")
    }
    self.top += 1;
    self.buf.push(elem);
  }
  
  pub fn pop(&mut self) -> Types {
     if self.top == 0 {
       panic!("Popping from empty stack")
     }
    self.top -= 1;
    self.buf.pop().unwrap()
  }
}

pub enum Types {
  Int(i32),
  Long(i64),
  Float(f32),
  Double(f64),
}