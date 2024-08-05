mod lex;
mod primitives;
mod parse;
mod ast;
mod span;
mod compiler;
mod element;
pub use primitives::glyph_list;

type Error = span::Span<String>;

pub fn compile(program: String) -> Result<String, String> {
    //println!("{program}");
    let tokens = lex::lex(program);
    //println!("{:?}", tokens);
    let ast = parse::parse(tokens)?;
    //println!("{:?}", ast);
    Ok(compiler::compile(ast.value).to_string())
    //println!("{}", element.to_string());
}
