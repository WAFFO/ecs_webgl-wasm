
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;


pub fn get_document() -> web_sys::Document {
    web_sys::window()
        .expect("Could not attach to window.")
        .document()
        .expect("Could not attach to document.")
}

pub fn get_canvas() -> Result<(web_sys::HtmlCanvasElement), JsValue> {
    let elm = get_document().get_element_by_id("canvas").unwrap();
    let canvas = elm.dyn_into::<web_sys::HtmlCanvasElement>()?;
    Ok(canvas)
}

pub fn log(s: &str) {
    console::log_1(&s.into());
}

pub fn log_1(s: &str, a: &JsValue) {
    console::log_2(&s.into(),&a);
}