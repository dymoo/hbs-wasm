extern crate handlebars;
extern crate html_minifier;

use handlebars::Handlebars;
use html_minifier::HTMLMinifier;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render(template: &str, ctx: &JsValue, minify: Option<bool>) -> String {
    let hbs = Handlebars::new();

    let json_value: handlebars::JsonValue = ctx.into_serde().unwrap();
    let mut rendered = hbs.render_template(template, &json_value).unwrap();

    if minify.unwrap_or(true) {
        let mut html_minifier = HTMLMinifier::new();

        html_minifier.digest(rendered).unwrap();
        rendered = str::from_utf8(html_minifier.get_html())
            .unwrap()
            .to_string();
    }

    return rendered;
}
