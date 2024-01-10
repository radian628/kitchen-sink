// we can implement other sizes in Kitchen Sink code
pub enum IntSize {
  I8,
  I16,
  I32,
  I64
}
pub enum FloatSize {
  F32,
  F64,
}

pub enum Instruction {
  // logical ops
  And(IntSize),
  Or(IntSize),
  Not(IntSize),

  // basic arithmetic
  Add(IntSize),
  Addf(FloatSize),
  Sub(IntSize),
  Subf(FloatSize),
  Mul(IntSize),
  Mulf(FloatSize),
  Div(IntSize),
  Divf(FloatSize),
  Mod(IntSize),
  Modf(FloatSize),

  // comparison (less than = -1, equal = 0, greater than = 1)
  Cmp(IntSize),
  Cmpf(FloatSize),

  // jumps (in theory we only need these two, though it'll be less efficient)
  Jmp(i64),
  // "jump if zero" instruction
  Jz(i64),

  // arguments: fd, dst pointer, size
  Read,
  // arguments:fd, src pointer, size
  Write,

  // push a byte to the stack (can be repeated to push larger types)
  Push(u8),

  // pop some number of bytes from the stack
  Pop(usize),

  // move some data to the top of the stack
  Mov(std::ptr, IntSize)
}