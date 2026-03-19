use crate::{OpCode, Value};

#[derive(Debug)]
pub struct VM {
    pub code: Vec<u8>,
    pc: usize,
    stack: Vec<Value>,
    vars: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            pc: 0,
            stack: Vec::new(),
            vars: Vec::new(),
        }
    }
    pub fn advance(&mut self) {
        self.pc += 1;
    }
    pub fn fetch(&mut self) -> u8 {
        let op = self.code[self.pc];

        self.pc += 1;
        op
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack Underflow")
    }

    fn next_u8(&mut self) -> u8 {
        let res = self.code[self.pc];

        self.pc += 1;
        res
    }

    fn next_u16(&mut self) -> u16 {
        let res = u16::from_le_bytes([self.code[self.pc], self.code[self.pc + 1]]);

        self.pc += 2;
        res
    }

    fn next_u32(&mut self) -> u32 {
        let res = u32::from_le_bytes([
            self.code[self.pc],
            self.code[self.pc + 1],
            self.code[self.pc + 2],
            self.code[self.pc + 3],
        ]);

        self.pc += 4;
        res
    }

    pub fn run(&mut self) {
        loop {
            println!("{:?}", self.stack);

            let op = self.fetch();

            match OpCode::try_from(op).unwrap() {
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a + b)),
                        _ => eprintln!("Operação invalida"),
                    }
                }
                OpCode::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a - b)),
                        _ => eprintln!("Operação invalida"),
                    }
                }
                OpCode::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a * b)),
                        _ => eprintln!("Operação invalida"),
                    }
                }
                OpCode::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a / b)),
                        _ => eprintln!("Operação invalida"),
                    }
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Dup => {
                    self.stack.push(*self.stack.last().unwrap());
                }
                OpCode::Greater => {
                    let b = self.pop();
                    let a = self.pop();
                    match (a, b) {
                        (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Bool(a > b)),
                        _ => eprintln!("Operação invalida"),
                    }
                }
                OpCode::LoadConst => {
                    let idx = self.next_u8();
                    let value = Value::Int(0);
                    self.stack.push(value);
                }
                OpCode::StoreLocal => {
                    let value = self.pop();
                    let idx = self.next_u8();
                    if self.vars.len() <= idx as usize {
                        self.vars.resize(idx as usize + 1, Value::Int(0));
                    }
                    self.vars[idx as usize] = value;
                }
                OpCode::LoadLocal => {
                    let idx = self.next_u8();
                    let value = self.vars[idx as usize];
                    self.stack.push(value);
                }
                OpCode::Print => println!("{:?}", self.pop()),
                OpCode::Halt => break,
            }
        }
    }
}
