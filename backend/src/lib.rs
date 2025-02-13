mod timehelp;
mod kalendar;
mod liturgy;
mod parser;
mod compiler;
mod lexer;
mod asset;
mod wasm;

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn get_identifier(year: i32, month: u32, day: u32) -> String {
    match wasm::get_identifier(year, month, day) {
        Ok(info) => serde_json::to_string(&info).unwrap(),
        Err(err) => serde_json::to_string(&err).unwrap(),
    }
}

#[wasm_bindgen]
pub fn get_monthly_identifiers(year: i32, month: u32) -> String {
    match wasm::get_monthly_identifiers(year, month) {
        Ok(map) => serde_json::to_string(&map).unwrap(),
        Err(err) => serde_json::to_string(&err).unwrap()
    }
}

#[wasm_bindgen]
pub fn get_hour(year: i32, month: u32, day: u32, hour: &str) -> String {
    match wasm::get_hour(year, month, day, hour) {
        Ok(elements) => serde_json::to_string(&elements).unwrap(),
        Err(err) => serde_json::to_string(&err).unwrap()
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("WASM initialized."));

    Ok(())
}