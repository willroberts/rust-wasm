mod utils;

extern crate console_error_panic_hook;

use chrono::Local;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn get_time() -> String {
    let now = Local::now();
    let time_string = now.format("The time is now %H:%M:%S!").to_string();
    time_string
}

#[wasm_bindgen]
pub fn greet() {
    console_error_panic_hook::set_once();
    let t = get_time();
    alert(t.as_str());
}
