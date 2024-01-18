// TODO: `Display` implementation for these types, mnemonics for instructions (i.e. XORB, XORH and such)

// we can implement other sizes in Kitchen Sink code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntSize {
    I8,
    I16,
    I32,
    I64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSize {
    F32,
    F64,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    // logical ops
    /// bitwise ANDs the last two items on the stack of the provided size, popping them and pushing the result
    And(IntSize),
    /// bitwise ORs the last two items on the stack of the provided size, popping them and pushing the result
    Or(IntSize),
    /// bitwise XORs the last two items on the stack of the provided size, popping them and pushing the result
    Xor(IntSize),
    /// bitwise NOTs the last item on the stack of the provided size, popping it and pushing the result
    Not(IntSize),
    /// pops the last two items from the stack, shifts the second to last one left by the amount given by the last one (filling with zeroes), and pushes the result
    Shl(IntSize),
    /// pops the last two items from the stack, shifts the second to last one right by the amount given by the last one (filling with sign bit), and pushes the result
    Shr(IntSize),
    /// pops the last two items from the stack, shifts the second to last one right by the amount given by the last one (filling with zeroes), and pushes the result
    UShr(IntSize),

    // basic arithmetic
    /// adds the last two integer items on the stack of the provided size, popping them and pushing the result
    Add(IntSize),
    /// adds the last two floating point items on the stack of the provided size, popping them and pushing the result
    Addf(FloatSize),
    /// subtracts the last two integer items on the stack of the provided size, popping them and pushing the result
    Sub(IntSize),
    /// subtracts the last two floating point items on the stack of the provided size, popping them and pushing the result
    Subf(FloatSize),
    /// multiplies the last two integer items on the stack of the provided size, popping them and pushing the result
    Mul(IntSize),
    /// multiplies the last two floating point items on the stack of the provided size, popping them and pushing the result
    Mulf(FloatSize),
    /// divides the last two integer items on the stack of the provided size, popping them and pushing the result
    Div(IntSize),
    /// divides the last two floating point items on the stack of the provided size, popping them and pushing the result
    Divf(FloatSize),
    /// modulos the last two integer items on the stack of the provided size, popping them and pushing the result
    Mod(IntSize),
    /// modulos the last two floating point items on the stack of the provided size, popping them and pushing the result
    Modf(FloatSize),

    /// compares the last two integer items on the stack of the provided size, popping them and pushing the result (less than = -1, equal = 0, greater than = 1)
    Cmp(IntSize),
    /// compares the last two floating point items on the stack of the provided size, popping them and pushing the result (less than = -1, equal = 0, greater than = 1)
    Cmpf(FloatSize),

    /// unconditionally jump to the given address
    Jmp(i64),
    /// pops the last item from the stack with the given size, then pops an address.  If the first value is zero, jumps the address
    Jz(IntSize),

    /// pops a 32 bit file descriptor, a destination address, and a 16 bit max size from the stack, then tries to read from the descriptor.  Returns the number of bytes read as a 16 bit integer, or -1 if there's an error
    Read,
    /// pops a 32 bit file descriptor, a source address, and a 16 bit max size from the stack, then tries to write to the descriptor.  Returns the number of bytes written as a 16 bit integer, or -1 if there's an error
    Write,

    /// push a byte to the stack (can be repeated to push larger types) (should we have it take a size param and allow for bigger payload?)
    Push(u8),

    /// pop some number of bytes from the stack
    Pop(usize),

    /// pop the last word from the stack, read `size` bytes of memory at that location, and push it to the stack
    Load { size: IntSize },
    /// pop the last word from the stack as an address, pop `size` more, and store that in memnory
    Store { size: IntSize },

    /// pushes the stack pointer (u64) to the stack
    PushSP,

    /// pushes the program counter (u64) to the stack
    PushIP,

    /// pushes the size of the heap in bytes (u64) to the stack
    PushMaxHeapSize
}
