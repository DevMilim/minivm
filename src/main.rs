use minivm::Lexer;

const CODE: &str = "
pub struct Person {
    name: String,
    age: u32,
}
impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}
fn main() {
    let person = Person::new(\"Milim\", 30);
}

";

fn main() {
    let lexer = Lexer::new(CODE);
    println!("{:#?}", lexer.lex_all());
}
