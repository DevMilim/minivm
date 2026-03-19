use crate::Value;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    LoadConst(Value),
    StoreLocal(u8),
    LoadLocal(u8),
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    Print,
    Pop,
    Dup,
    Halt,
}
