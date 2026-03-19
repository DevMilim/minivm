use minivm::{OpCode, VM, Value};

fn main() {
    let mut vm = VM::new();
    vm.code = vec![];
    vm.run();
}
