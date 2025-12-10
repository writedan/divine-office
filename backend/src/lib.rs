mod asset;
mod gabc;
mod kalendar;
mod lexer;
mod parser;
mod runtime;
mod timehelp;
mod wasm;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_identifiers(year: i32, month: u32, day: u32) -> String {
    let date = match wasm::from_ymd(year, month, day) {
        Ok(d) => d,
        Err(why) => return serde_json::to_string(&Err::<(), String>(why)).unwrap(),
    };

    match wasm::get_identifiers(date) {
        Ok(info) => serde_json::to_string(&info).unwrap(),
        Err(err) => serde_json::json!([{ "Error": format!("Failed to get identifiers: {}", err) }])
            .to_string(),
    }
}

#[wasm_bindgen]
pub fn get_monthly_identifiers(year: i32, month: u32) -> String {
    match wasm::get_monthly_identifiers(year, month) {
        Ok(map) => serde_json::to_string(&map).unwrap(),
        Err(err) => {
            serde_json::json!([{ "Error": format!("Failed to get monthly identifiers: {}", err) }])
                .to_string()
        }
    }
}

#[wasm_bindgen]
pub fn get_hour(celebration: &str, hour: &str) -> String {
    let celebration = serde_json::from_str(celebration).unwrap();

    match wasm::get_hour(celebration, hour) {
        Ok(elements) => serde_json::to_string(&elements).unwrap(),
        Err(err) => {
            serde_json::json!([{ "Error": format!("Failed to get hour: {}", err) }]).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn has_first_vespers(today: &str, tomorrow: &str) -> bool {
    let (today, tomorrow) = (
        serde_json::from_str(today).unwrap(),
        serde_json::from_str(tomorrow).unwrap(),
    );
    wasm::has_first_vespers(today, tomorrow)
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once(); // converts panics to console.error
}