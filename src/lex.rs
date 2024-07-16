enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Operator(char),
    Number(u32),
    Str(String),
    Name(String),
    Whitespace,
}

struct Lexer {
    program: String,
    index: usize,
}

impl Lexer {
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.next() {
           tokens.push(match ch {
               '[' => Token::OpenBracket,
               ']' => Token::CloseBracket,
               '"' => Token::Str(self.string()),
               _ => Token::Whitespace,
           })
        } 
        tokens
    } 

    pub fn string(&mut self) -> String {
        String::new()
    } 

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        self.program.chars().nth(self.index - 1)
    } 
}
