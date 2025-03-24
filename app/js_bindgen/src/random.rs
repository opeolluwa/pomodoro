use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/random.js")]
extern "C" {
    pub fn get_random_uint(max: usize) -> usize;
}
