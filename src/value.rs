#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Value {
    Int(i32),
    Bool(bool),
}

impl Value {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Value::Int(i) => i.to_le_bytes().to_vec(),
            Value::Bool(b) => vec![b as u8],
        }
    }
}
