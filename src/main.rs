mod lex;
mod primitives;
mod span;

fn main() {
    let program = include_str!("../example.web").to_string();

    println!("{:?}", lex::lex(program));
}

