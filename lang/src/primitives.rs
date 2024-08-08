use std::fmt::{Display, Write};
use std::ops::Deref;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlyphType {
    Html,
    Css,
    Js,
    Math,
    Bracket,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Glyph {
    pub glyph: char,
    pub glyph_type: GlyphType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GlyphInfo {
    pub glyph: char,
    pub attr: String,
    pub desc: String,
    pub category: String,
    pub glyph_type: GlyphType,
}

pub struct GlyphLoader {
    glyphs: HashMap<char, GlyphInfo>
}

impl GlyphLoader {
    pub fn setup() -> Self {
        let mut glyphs = HashMap::new(); 
        hash_glyphs(&mut glyphs, load_glyphs("html.glyph", GlyphType::Html));
        hash_glyphs(&mut glyphs, load_glyphs("css.glyph", GlyphType::Css));
        hash_glyphs(&mut glyphs, load_glyphs("js.glyph", GlyphType::Js));
        hash_glyphs(&mut glyphs, load_glyphs("math.glyph", GlyphType::Math));
        Self { glyphs }
    }

    pub fn hashmap() -> HashMap<char, GlyphInfo> {
        let gl = Self::setup();
        gl.glyphs
    }

    pub fn match_tok(&self, ch: char) -> Glyph {
        let GlyphInfo { glyph, glyph_type, .. } = self.glyphs[&ch];
        Glyph { glyph, glyph_type }
    }
}

impl Deref for GlyphLoader {
    type Target = HashMap<char, GlyphInfo>;

    fn deref(&self) -> &Self::Target {
        &self.glyphs
    }
}

pub fn hash_glyphs(hashmap: &mut HashMap<char, GlyphInfo>, glyphs: Vec<GlyphInfo>) {
    for glyph in glyphs {
        hashmap.insert(glyph.glyph, glyph);
    }
}

pub fn load_glyphs(name: &str, glyph_type: GlyphType) -> Vec<GlyphInfo> {
    let file = std::fs::read_to_string("../../".to_owned() + name).unwrap(); 
    let lines = file.lines();
    let mut category = "";
    let mut glyphs = Vec::new();

    for line in lines {
        if line.starts_with("*") {
            category = &line[2..];
        }

        let split = line.split('\\').collect::<Vec<&str>>();
        if split.len() >= 3 {
            glyphs.push(GlyphInfo {
                glyph: split[0].chars().next().unwrap(), 
                attr: split[1].to_string(), 
                desc: split[2].to_string(),
                category: category.to_string(),
                glyph_type
            });
        }
    }

    glyphs
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.glyph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_css() {
        panic!("{:#?}", load_glyphs("../css.glyph"));
    }
}
