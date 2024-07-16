mod lex;

fn main() {
    let program = include_str!("../example.web").to_string();

    println!("{:?}", Parser { program, index: 0 }.parse());
}

