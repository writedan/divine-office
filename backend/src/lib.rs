mod asset;
mod kalendar;
mod timehelp;
mod wasm;
mod lexer;
mod parser;
mod runtime;
mod gabc;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_identifiers(year: i32, month: u32, day: u32) -> String {
	let date = match wasm::from_ymd(year, month, day) {
		Ok(d) => d,
		Err(why) => return serde_json::to_string(&Err::<(), String>(why)).unwrap()
	};

    match wasm::get_identifiers(date) {
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
pub fn get_hour(celebration: &str, hour: &str) -> String {
	let celebration = serde_json::from_str(celebration).unwrap();

    match wasm::get_hour(celebration, hour) {
        Ok(elements) => serde_json::to_string(&elements).unwrap(),
        Err(err) => serde_json::to_string(&err).unwrap(),
    }
}
