use minivm::{OpCode, VM, Value};

fn main() {
    let mut vm = VM::new();
    vm.code = vec![
        OpCode::LoadConst(Value::Int(10)),
        OpCode::StoreLocal(0),
        OpCode::LoadConst(Value::Int(5)),
        OpCode::StoreLocal(1),
        OpCode::LoadLocal(0),
        OpCode::LoadLocal(1),
        OpCode::Greater,
        OpCode::Print,
        OpCode::Halt,
    ];
    vm.run();
}
