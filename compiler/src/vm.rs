use std::io::{stdin, Read, stdout, Write};

use crate::bytecode::Instruction;
use crate::bytecode::{IntSize, FloatSize};

pub struct VM {
    program: Vec<Instruction>,
    main_memory: Vec<u8>,
    stack: Vec<u8>,
    stack_pointer: u64,
    program_counter: u64,
    max_stack: u64
}

pub enum Fault {
    SegmentationFault, StackOverflow, ProgramEnded
}

impl VM {
    pub const STACK_START: u64 = 0x1000000000000000;

    // boilerplate garbage

    // we might have to change this to return a vec for borrow checker reasons
    pub fn get_bytes(&self, addr: u64, count: u64) -> Result<&[u8], Fault> {
        let end = match addr.checked_add(count) {
            Some(v) => v,
            None => return Err(Fault::SegmentationFault),
        };
        if end < self.main_memory.len() as u64 {
            Ok(&self.main_memory[addr as usize..end as usize])
        } else if addr > Self::STACK_START && end < Self::STACK_START + self.stack.len() as u64 {
            Ok(&self.stack[(addr - Self::STACK_START) as usize..(end - Self::STACK_START) as usize])
        } else {
            Err(Fault::SegmentationFault)
        }
    }

    pub fn set_bytes(&mut self, addr: u64, bytes: &[u8]) -> Result<(), Fault> {
        let end = match addr.checked_add(bytes.len() as u64) {
            Some(v) => v,
            None => return Err(Fault::SegmentationFault),
        };
        if end < self.main_memory.len() as u64 {
            let slice = &mut self.main_memory[addr as usize..end as usize];
            slice.copy_from_slice(bytes);
            Ok(())
        } else if addr > Self::STACK_START && end < Self::STACK_START + self.stack.len() as u64 {
            let slice = &mut self.main_memory[(addr - Self::STACK_START) as usize..(end - Self::STACK_START) as usize];
            slice.copy_from_slice(bytes);
            Ok(())
        } else {
            Err(Fault::SegmentationFault)
        }
    }

    pub fn get_u8(&self, addr: u64) -> Result<u8, Fault> { Ok(self.get_bytes(addr, 1)?[0]) }
    pub fn get_u16(&self, addr: u64) -> Result<u16, Fault> { let b = self.get_bytes(addr, 2)?; Ok(u16::from_le_bytes([b[0], b[1]])) }
    pub fn get_u32(&self, addr: u64) -> Result<u32, Fault> { let b = self.get_bytes(addr, 4)?; Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]])) }
    pub fn get_u64(&self, addr: u64) -> Result<u64, Fault> { let b = self.get_bytes(addr, 8)?; Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])) }
    pub fn get_f32(&self, addr: u64) -> Result<f32, Fault> { Ok(f32::from_bits(self.get_u32(addr)?)) }
    pub fn get_f64(&self, addr: u64) -> Result<f64, Fault> { Ok(f64::from_bits(self.get_u64(addr)?)) }

    pub fn set_u8(&mut self, addr: u64, value: u8) -> Result<(), Fault> { self.set_bytes(addr, &[value]) }
    pub fn set_u16(&mut self, addr: u64, value: u16) -> Result<(), Fault> { self.set_bytes(addr, &value.to_le_bytes()) }
    pub fn set_u32(&mut self, addr: u64, value: u32) -> Result<(), Fault> { self.set_bytes(addr, &value.to_le_bytes()) }
    pub fn set_u64(&mut self, addr: u64, value: u64) -> Result<(), Fault> { self.set_bytes(addr, &value.to_le_bytes()) }
    pub fn set_f32(&mut self, addr: u64, value: f32) -> Result<(), Fault> { self.set_u32(addr, value.to_bits()) }
    pub fn set_f64(&mut self, addr: u64, value: f64) -> Result<(), Fault> { self.set_u64(addr, value.to_bits()) }

    // we might have to change this to return a vec for borrow checker reasons
    pub fn pop_bytes(&mut self, count: u64) -> Result<&[u8], Fault> {
        let from = self.stack_pointer.checked_sub(count).ok_or(Fault::SegmentationFault)?;
        let to = self.stack_pointer;
        let bytes = &self.stack[(from - Self::STACK_START) as usize..(to - Self::STACK_START) as usize];

        self.stack_pointer = from;

        Ok(bytes)
    }

    pub fn pop_u8(&mut self) -> Result<u8, Fault> { Ok(self.pop_bytes(1)?[0]) }
    pub fn pop_u16(&mut self) -> Result<u16, Fault> { let b = self.pop_bytes(2)?; Ok(u16::from_le_bytes([b[0], b[1]])) }
    pub fn pop_u32(&mut self) -> Result<u32, Fault> { let b = self.pop_bytes(4)?; Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]])) }
    pub fn pop_u64(&mut self) -> Result<u64, Fault> { let b = self.pop_bytes(8)?; Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])) }
    pub fn pop_f32(&mut self) -> Result<f32, Fault> { Ok(f32::from_bits(self.pop_u32()?)) }
    pub fn pop_f64(&mut self) -> Result<f64, Fault> { Ok(f64::from_bits(self.pop_u64()?)) }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), Fault> {
        self.ensure_stack(bytes.len() as u64)?;

        let start = (self.stack_pointer - Self::STACK_START) as usize;
        let end = (self.stack_pointer + bytes.len() as u64 - Self::STACK_START) as usize;

        self.stack[start..end].copy_from_slice(bytes);

        Ok(())
    }

    pub fn push_u8(&mut self, value: u8) -> Result<(), Fault> { self.push_bytes(&[value]) }
    pub fn push_u16(&mut self, value: u16) -> Result<(), Fault> { self.push_bytes(&value.to_le_bytes()) }
    pub fn push_u32(&mut self, value: u32) -> Result<(), Fault> { self.push_bytes(&value.to_le_bytes()) }
    pub fn push_u64(&mut self, value: u64) -> Result<(), Fault> { self.push_bytes(&value.to_le_bytes()) }
    pub fn push_f32(&mut self, value: f32) -> Result<(), Fault> { self.push_u32(value.to_bits()) }
    pub fn push_f64(&mut self, value: f64) -> Result<(), Fault> { self.push_u64(value.to_bits()) }

    pub fn push_i64(&mut self, value: i64) -> Result<(), Fault> { self.push_u64(value as u64) }

    pub fn ensure_stack(&mut self, required_size: u64) -> Result<(), Fault> {
        let current_usage = self.stack_pointer - Self::STACK_START;
        if current_usage + required_size >= self.max_stack {
            return Err(Fault::StackOverflow);
        }
        if (self.stack.len() as u64) < current_usage + required_size {
            return Ok(());
        }
        for _ in 0..(current_usage + required_size) - self.stack.len() as u64 {
            self.stack.push(0);
        }
        Ok(())
    }

    // actual code
    pub fn tick(&mut self) -> Result<(), Fault> {
        let pc = self.program_counter;
        let instruction = match self.program.get(pc as usize) {
            Some(v) => v,
            None => return Err(Fault::ProgramEnded),
        };
        self.program_counter += 1;

        macro_rules! sizes {
            (int biop $size:expr; $a:ident, $b:ident => $action:expr) => {
                match $size {
                    IntSize::I8 => { let $b = self.pop_u8()?; let $a = self.pop_u8()?; self.push_u8($action)?; }
                    IntSize::I16 => { let $b = self.pop_u16()?; let $a = self.pop_u16()?; self.push_u16($action)?; }
                    IntSize::I32 => { let $b = self.pop_u32()?; let $a = self.pop_u32()?; self.push_u32($action)?; }
                    IntSize::I64 => { let $b = self.pop_u64()?; let $a = self.pop_u64()?; self.push_u64($action)?; }
                }
            };
            (int biop $size:expr; $a:ident, $b:ident => $ret:ident $action:expr) => {
                match $size {
                    IntSize::I8 => { let $b = self.pop_u8()?; let $a = self.pop_u8()?; self.$ret($action)?; }
                    IntSize::I16 => { let $b = self.pop_u16()?; let $a = self.pop_u16()?; self.$ret($action)?; }
                    IntSize::I32 => { let $b = self.pop_u32()?; let $a = self.pop_u32()?; self.$ret($action)?; }
                    IntSize::I64 => { let $b = self.pop_u64()?; let $a = self.pop_u64()?; self.$ret($action)?; }
                }
            };
            (signed int biop $size:expr; $a:ident, $b:ident => $action:expr) => {
                match $size {
                    IntSize::I8 => { let $b = self.pop_u8()? as i8; let $a = self.pop_u8()? as i8; self.push_u8(($action) as u8)?; }
                    IntSize::I16 => { let $b = self.pop_u16()? as i16; let $a = self.pop_u16()? as i16; self.push_u16(($action) as u16)?; }
                    IntSize::I32 => { let $b = self.pop_u32()? as i32; let $a = self.pop_u32()? as i32; self.push_u32(($action) as u32)?; }
                    IntSize::I64 => { let $b = self.pop_u64()? as i64; let $a = self.pop_u64()? as i64; self.push_u64(($action) as u64)?; }
                }
            };
            (float biop $size:expr; $a:ident, $b:ident => $action:expr) => {
                match $size {
                    FloatSize::F32 => { let $b = self.pop_f32()?; let $a = self.pop_f32()?; self.push_f32($action)?; }
                    FloatSize::F64 => { let $b = self.pop_f64()?; let $a = self.pop_f64()?; self.push_f64($action)?; }
                }
            };
            (float biop $size:expr; $a:ident, $b:ident => $ret:ident $action:expr) => {
                match $size {
                    FloatSize::F32 => { let $b = self.pop_f32()?; let $a = self.pop_f32()?; self.$ret($action)?; }
                    FloatSize::F64 => { let $b = self.pop_f64()?; let $a = self.pop_f64()?; self.$ret($action)?; }
                }
            };
        }

        match instruction {
            Instruction::And(size) => sizes!(int biop size; a, b => a + b),
            Instruction::Or(size) => sizes!(int biop size; a, b => a | b),
            Instruction::Xor(size) => sizes!(int biop size; a, b => a ^ b),
            Instruction::Not(size) => match size {
                IntSize::I8 => { let v = self.pop_u8()?; self.push_u8(!v)?; }
                IntSize::I16 => { let v = self.pop_u16()?; self.push_u16(!v)?; }
                IntSize::I32 => { let v = self.pop_u32()?; self.push_u32(!v)?; }
                IntSize::I64 => { let v = self.pop_u64()?; self.push_u64(!v)?; }
            },
            Instruction::Shl(size) => sizes!(int biop size; a, b => a << b),
            Instruction::Shr(size) => sizes!(signed int biop size; a, b => a >> b),
            Instruction::UShr(size) => sizes!(int biop size; a, b => a >> b),
            Instruction::Add(size) => sizes!(int biop size; a, b => a + b),
            Instruction::Addf(size) => sizes!(float biop size; a, b => a + b),
            Instruction::Sub(size) => sizes!(int biop size; a, b => a - b),
            Instruction::Subf(size) => sizes!(float biop size; a, b => a - b),
            Instruction::Mul(size) => sizes!(int biop size; a, b => a * b),
            Instruction::Mulf(size) => sizes!(float biop size; a, b => a * b),
            Instruction::Div(size) => sizes!(int biop size; a, b => a / b),
            Instruction::Divf(size) => sizes!(float biop size; a, b => a / b),
            Instruction::Mod(size) => sizes!(int biop size; a, b => a % b),
            Instruction::Modf(size) => sizes!(float biop size; a, b => a % b),
            Instruction::Cmp(size) => {
                sizes!(int biop size; a, b => push_i64 match &a.cmp(&b) {  // TODO: i64?
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                });
            },
            Instruction::Cmpf(size) => {
                sizes!(float biop size; a, b => push_i64 match &a.partial_cmp(&b) {  // TODO: i64?
                    Some(std::cmp::Ordering::Less) => -1,
                    Some(std::cmp::Ordering::Equal) => 0,
                    Some(std::cmp::Ordering::Greater) => 1,
                    None => todo!("float compare failed, TODO: figure out return for this, or fault?")
                });
            },
            Instruction::Jmp(addr) => { self.program_counter = *addr as u64; }
            Instruction::Jz(size) => {
                let v = match size {
                    IntSize::I8 => self.pop_u8()? as u64,
                    IntSize::I16 => self.pop_u16()? as u64,
                    IntSize::I32 => self.pop_u32()? as u64,
                    IntSize::I64 => self.pop_u64()? as u64,
                };
                let addr = self.pop_u64()?;
                if v == 0 {
                    self.program_counter = addr;
                }
            }
            Instruction::Read => {
                let fd = self.pop_u32()?;
                let dst_start = self.pop_u64()?;
                let max_len = self.pop_u16()?;
                let mut buf = vec![0; max_len as usize];

                match fd {
                    0 => {
                        // TODO: test harness
                        let read = match stdin().read(buf.as_mut_slice()) {
                            Ok(v) => { self.set_bytes(dst_start, &buf[0..v])?; v as u16 }
                            Err(_) => { u16::MAX }
                        };
                        self.push_u16(read)?;
                    }
                    _ => todo!("reading from file descriptors other than 0 (stdin) is not yet supported")
                }
            }
            Instruction::Write => {
                let fd = self.pop_u32()?;
                let src_start = self.pop_u64()?;
                let max_len = self.pop_u16()?;
                let buf = self.get_bytes(src_start, max_len as u64)?;

                match fd {
                    1 => {
                        // TODO: test harness
                        let written = match stdout().write(&buf) {
                            Ok(v) => v as u16,
                            Err(_) => u16::MAX,
                        };
                        self.push_u16(written)?;
                    }
                    _ => todo!("writing to file descriptors other than 1 (stdout) is not yet supported")
                }
            }
            Instruction::Push(v) => self.push_u8(*v)?,
            Instruction::Pop(n) => { self.pop_bytes(*n as u64)?; }
            Instruction::Load { size } => { let s = *size; let addr = self.pop_u64()?; match s {
                IntSize::I8 => self.push_u8(self.get_u8(addr)?)?,
                IntSize::I16 => self.push_u16(self.get_u16(addr)?)?,
                IntSize::I32 => self.push_u32(self.get_u32(addr)?)?,
                IntSize::I64 => self.push_u64(self.get_u64(addr)?)?,
            } }
            Instruction::Store { size } => { let s = *size; let addr = self.pop_u64()?; match s {
                IntSize::I8 => { let v = self.pop_u8()?; self.set_u8(addr, v)? }
                IntSize::I16 => { let v = self.pop_u16()?; self.set_u16(addr, v)? }
                IntSize::I32 => { let v = self.pop_u32()?; self.set_u32(addr, v)? }
                IntSize::I64 => { let v = self.pop_u64()?; self.set_u64(addr, v)? }
            } }
            Instruction::PushSP => { self.push_u64(self.stack_pointer)? }
            Instruction::PushIP => { self.push_u64(self.program_counter)? }
            Instruction::PushMaxHeapSize => { self.push_u64(self.main_memory.len() as u64)? }
        }

        Ok(())
    }
}
