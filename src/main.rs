use minivm::VM;

fn main() {
    let mut vm = VM::new();
    vm.code = vec![];
    vm.run();
}
