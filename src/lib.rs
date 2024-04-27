pub mod app;
pub mod element;
pub mod event;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(a: &str);
    #[wasm_bindgen]
    pub fn alert(a: &str);
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! alert {
    ($($t:tt)*) => (alert(&format_args!($($t)*).to_string()))
}
