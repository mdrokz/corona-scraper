#[macro_use]
extern crate serde_derive;

mod utils;

mod types;

use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s:&str);
}

#[wasm_bindgen]
pub async fn getCoronaData() -> String {
    log("running");
    let data = getData().await;
    data
}

async fn getData() -> String {
    let res = reqwest::get("https://corona.lmao.ninja/countries").await.unwrap();
    res.text().await.unwrap()
}