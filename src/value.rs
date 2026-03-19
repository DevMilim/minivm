#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Value {
    Int(i32),
    Bool(bool),
}
