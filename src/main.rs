use crate::{lexer::Lexer, token::Token};

mod lexer;
mod token;

fn main() {
    let input = r#"
    let a=123+233-333*555/555;
    fn aaa(){
        if ( 12==22){}
        return 123;
    }"#
        .to_string();
    let mut lexer = Lexer::new(input.clone());

    loop {
        let token = lexer.next_token();
        if token.r#type == token::EOF {
            break;
        }

        println!("{:?}", token);
    }

}
