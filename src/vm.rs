use crate::{OpCode, Value};

#[derive(Debug)]
pub struct VM {
    pub code: Vec<OpCode>,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            ip: 0,
            stack: Vec::new(),
        }
    }
    pub fn advance(&mut self) {
        self.ip += 1;
    }
    pub fn fetch(&mut self) -> OpCode {
        let op = self.code[self.ip];

        self.ip += 1;
        op
    }
    pub fn run(&mut self) {
        loop {
            println!("{:?}", self.stack);

            let op = self.fetch();

            match op {
                OpCode::LoadIntConst(i) => self.stack.push(Value::Int(i)),
                OpCode::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a + b)),
                    }
                }
                OpCode::Halt => break,
            }
        }
    }
}
