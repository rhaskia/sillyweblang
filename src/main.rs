mod lex;
mod primitives;
mod parse;
mod ast;
mod span;
mod compiler;
mod element;

fn main() {
    let program = include_str!("../example.web").to_string();

    let tokens = lex::lex(program);
    println!("{:?}", tokens);
    let ast = parse::parse(tokens);
    println!("{:?}", ast);
    let element = compiler::compile(ast.unwrap().value);
    println!("{}", element.to_string());
}
