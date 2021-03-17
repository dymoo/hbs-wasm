extern crate handlebars;
extern crate html_minifier;

use handlebars::Handlebars;
use html_minifier::HTMLMinifier;
use serde_json::Value;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(template: &str, ctx: &str, minify: bool) -> String {
    let hbs = Handlebars::new();
    let mut html_minifier = HTMLMinifier::new();

    let v: Value = serde_json::from_str(ctx).unwrap();
    let mut rendered = hbs.render_template(template, &v).unwrap();

    if minify {
        html_minifier.digest(rendered).unwrap();
        rendered = str::from_utf8(html_minifier.get_html())
            .unwrap()
            .to_string();
    }

    return rendered;
}
