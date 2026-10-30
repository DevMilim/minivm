Quero criar um compilador basico para rust que e capaz de gerar binario para:
psx
chip-8
Não vai ser possivel fazer completo porque rust tem muita coisa então vou fazer coisas basicas e tentar permitir ffi para facilitar o desenvolvimento
Alvos de pesquisa:
Lexer
Parser
AST
RI
LLVM
Back-end proprio
codigo minimo inicial:
``` rust
const VALUES: &str = "HELLO";

pub enum Values {
    X(i32),
    Y(u32),
    Pos { x: i32, y: i32 },
    None,
}

pub struct Person {
    name: &'static str,
    age: i32,
}

impl Person {
    pub fn get_age(&self) -> i32 {
        self.age
    }
}

fn main() {
    let x = 12;
    let y = 12;
    let person = Person {
        name: "Adam",
        age: 21234,
    };
    let adam_age = person.get_age();
    let list = [1, 2, 3, 4, 5];
    let mut total = 0;
    for i in list {
        total += i;
    }
    if let Values::X(i) = X(12) {
        i;
    }
}

```