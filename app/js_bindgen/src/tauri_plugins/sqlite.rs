use wasm_bindgen::prelude::*;

// create rust functions from the javascript functions
#[wasm_bindgen(js_namespace = ["window", "__TAURI__", "sql"])]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "sql"])]
    pub type Database;

    #[wasm_bindgen(constructor)]
    pub async fn load(database_path: &str) -> Database;

    #[wasm_bindgen(method)]
    pub async fn execute(this: &Database, query: &str, params: Vec<String>) -> JsValue;

}
