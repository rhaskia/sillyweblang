fn main() {
    let program = include_str!("../example.web").to_string();

    println!("{:?}", Parser { program, index: 0 }.parse());
}

struct Parser {
    program: String,
    index: usize,
}

impl Parser {
    pub fn parse(&mut self) -> NestedTokens {
       self.expr() 
    }

    pub fn expr(&mut self) -> NestedTokens {
        let mut expr = String::new();
        let mut children = Vec::new();
        while let Some(ch) = self.next() {
           match ch {
               '[' => children.push(self.expr()),
               ']' => break,
               _ => expr.push(ch),
           }
        } 
        NestedTokens::Element(expr, children)
    } 

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        self.program.chars().nth(self.index - 1)
    } 
}

#[derive(Debug)]
enum NestedTokens {
    Children(String),
    Element(String, Vec<NestedTokens>),
}
