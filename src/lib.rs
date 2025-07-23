mod utils;

extern crate console_error_panic_hook;

use chrono::Local;
use wasm_bindgen::prelude::*;
use web_sys::{Window,Document};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn get_time() -> String {
    let now = Local::now();
    let time_string = now.format("<center><h1>The time is now %H:%M:%S!</h1></center>").to_string();
    time_string
}

#[wasm_bindgen]
pub fn change_title(s: &str) {
    let window = web_sys::window().expect("window should exist in current context");
    let document = window.document().expect("document should exist in current context");
    document.set_title(s);
}

#[wasm_bindgen]
pub fn change_body_content(s: &str) {
    let window = web_sys::window().expect("window should exist in current context");
    let document = window.document().expect("document should exist in current context");
    let body = document.body().expect("body should exist in current context");
    body.set_inner_html(s);
}

#[wasm_bindgen]
pub fn show_alert(s: &str) {
    alert(s);
}

#[wasm_bindgen]
pub fn show_time() {
    console_error_panic_hook::set_once();
    let t = get_time();
    change_body_content(t.as_str());
}
