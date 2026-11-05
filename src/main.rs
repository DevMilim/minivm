use minivm::Lexer;

const CODE: &str = include_str!("teste.rs");

fn main() {
    let lexer = Lexer::new(CODE);
    match lexer.lex_all() {
        Ok(lexed) => {
            let code_string = CODE.to_string();
            println!("{}", code_string);
            for token in lexed {
                let span = token.span;
                let span = &code_string[span.start..span.end];
                println!("Text: {:?} Token: {:?}", span, token.kind);
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
