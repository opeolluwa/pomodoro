use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/prev_location.js")]
extern "C" {
    pub fn go_to_prev_location();
}
