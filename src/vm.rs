use std::collections::HashMap;

use crate::{OpCode, Value};

#[derive(Debug)]
pub struct VM {
    pub code: Vec<OpCode>,
    ip: usize,
    stack: Vec<Value>,
    vars: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            ip: 0,
            stack: Vec::new(),
            vars: Vec::new(),
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

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Stack Underflow")
    }

    pub fn run(&mut self) {
        loop {
            println!("{:?}", self.stack);

            let op = self.fetch();

            match op {
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
                OpCode::LoadConst(v) => {
                    self.stack.push(v);
                }
                OpCode::StoreLocal(idx) => {
                    let value = self.pop();
                    if self.vars.len() <= idx as usize {
                        self.vars.resize(idx as usize + 1, Value::Int(0));
                    }
                    self.vars[idx as usize] = value;
                }
                OpCode::LoadLocal(idx) => {
                    let value = self.vars[idx as usize];
                    self.stack.push(value);
                }
                OpCode::Print => println!("{:?}", self.pop()),
                OpCode::Halt => break,
            }
        }
    }
}
