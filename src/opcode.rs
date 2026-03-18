#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    LoadIntConst(i32),
    Add,
    Halt,
}
