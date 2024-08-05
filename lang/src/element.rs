use std::collections::HashMap;
use crate::compiler::Value;

#[derive(Debug)]
pub struct Element {
    tag: String,
    css_attr: HashMap<String, Value>,
    actions: HashMap<String, String>,
    children: Vec<Element>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Self { 
            tag: tag.to_string(),
            css_attr: HashMap::new(),
            actions: HashMap::new(),
            children: Vec::new()
        }
    }

    pub fn style(&mut self, name: &str, value: Value) {
        self.css_attr.insert(name.to_string(), value);
    }

    pub fn with_attr(mut self, name: &str, value: Value) -> Self {
        self.css_attr.insert(name.to_string(), value);
        self
    }

    pub fn with_children(mut self, mut children: Vec<Element>) -> Self {
        self.children.append(&mut children);
        self
    }

    pub fn add_call(&mut self, name: &str, value: &str) {
        self.actions.insert(name.to_string(), value.to_string());
    }

    pub fn add(&mut self, child: Element) {
        self.children.push(child);
    }

    pub fn to_html(&self, mut depth: usize) -> String {
        depth += 1;
        let indent = "    ".repeat(depth - 1);
        if self.children.is_empty() {
            format!("{0} <{1} style=\"{2}\"></{1}>", indent, self.tag, self.css())
        } else {
            format!("{0}<{1} style=\"{2}\">\n{3}\n{0}</{1}>", 
                indent,
                self.tag, 
                self.css(),
                self.children.iter()
                    .map(|c| c.to_html(depth))
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        }
    } 

    pub fn css(&self) -> String {
        self.css_attr.iter().map(|(k, v)| format!("{k}: {}", v.to_string())).collect::<Vec<String>>().join(" ")
    }
}

impl ToString for Element {
    fn to_string(&self) -> String {
        self.to_html(0)
    }
}
