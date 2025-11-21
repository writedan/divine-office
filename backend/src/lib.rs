mod asset;
mod compiler;
mod kalendar;
mod lexer;
mod liturgy;
mod parser;
mod timehelp;
mod wasm;

use wasm_bindgen::prelude::*;

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
        Err(err) => serde_json::to_string(&err).unwrap(),
    }
}

#[wasm_bindgen]
pub fn get_hour(year: i32, month: u32, day: u32, hour: &str) -> String {
    match wasm::get_hour(year, month, day, hour) {
        Ok(elements) => serde_json::to_string(&elements).unwrap(),
        Err(err) => serde_json::to_string(&err).unwrap(),
    }
}