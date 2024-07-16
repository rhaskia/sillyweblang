pub fn lex(program: String) -> Vec<Token> {
    Lexer { program, index: 0 }.lex()
}

#[derive(Debug)]
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
               '(' => Token::OpenParen,
               ')' => Token::CloseParen,
               '{' => Token::OpenBrace,
               '}' => Token::CloseBrace,
               '"' => Token::Str(self.string()),
               _ => Token::Whitespace,
           })
        } 
        tokens
    } 

    pub fn string(&mut self) -> String {
        let mut s = String::new();
        while let Some(ch) = self.next() {
            if ch == '"' { break }
            s.push(ch);
        }
        s
    } 

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        self.program.chars().nth(self.index - 1)
    } 
}
