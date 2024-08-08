use dioxus::prelude::*;
use weblang::GlyphLoader;
use weblang::GlyphInfo;

fn main() {
    dioxus::launch(App)
}

#[component]
pub fn App() -> Element {
    let mut user_text = use_signal(|| String::new());
    let mut compiled: Signal<Result<String, String>> = use_signal(|| Ok(String::new()));
    let glyph_loader = use_signal(GlyphLoader::setup);
    let glyphs = use_signal(|| GlyphLoader::hashmap());

    rsx!{
        div {
            class: "operatorbar",
            for (char, glyph) in glyphs.read().clone() {
                button {
                    onclick: move |_| user_text.write().push(char),
                    "{char}"
                }
            }
        }

        textarea {
            onchange: move |v| {
                compiled.set(weblang::compile(v.value()));
                user_text.set(v.value());
            },
            value: user_text,
        }

        match compiled() {
            Ok(html) => rsx!{
                div {
                    width: "300px",
                    height: "300px",
                    border: "1px solid",
                    dangerous_inner_html: html,
                }
                div {
                    "{html}"
                }
            },
            Err(err) => rsx!{
                div {
                    "{err}",
                }
            }
        }
    }
}

