#[repr(u8)]
#[derive(Clone, Copy, Debug)]
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

impl TryFrom<u8> for OpCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let values = match value {
            0 => OpCode::Add,
            1 => OpCode::Sub,
            _ => return Err(()),
        };
        Ok(values)
    }
}

pub struct Chunk {}
