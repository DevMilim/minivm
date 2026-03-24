use macros::FromRepr;

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromRepr)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    LoadConst,
    StoreLocal,
    LoadLocal,
    Print,
    Pop,
    Dup,
    Halt,
}
