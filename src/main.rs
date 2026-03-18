use minivm::{OpCode, VM};

fn main() {
    let mut vm = VM::new();
    vm.code = vec![
        OpCode::LoadIntConst(2),
        OpCode::LoadIntConst(3),
        OpCode::Add,
        OpCode::Halt,
    ];
    vm.run();
}
