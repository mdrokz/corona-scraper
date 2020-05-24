#[macro_use]
extern crate serde_derive;

mod utils;

mod types;

use wasm_bindgen::prelude::*;

use scraper::{Html, Selector};

use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn getCoronaData() -> String {
    log("Rust Assembly Running");
    let data = getData().await;
    data
}

async fn getData() -> String {
    let res = reqwest::get("https://disease.sh/v2/countries")
        .await
        .unwrap();
    res.text().await.unwrap()
}

#[wasm_bindgen]
pub struct Vtuple(Vec<String>, Vec<String>);

#[wasm_bindgen]
pub struct Vlength(pub usize, pub usize);

// #[wasm_bindgen]
// pub struct VData(pub Box<[u8]>, pub &'static str);

#[wasm_bindgen]
impl Vtuple {
    pub fn get_length(&self) -> Vlength {
        let h = self.0.len();
        let p = self.1.len();
        //  let p_inner = self.1[index].clone().len();
        Vlength(h, p)
    }

    pub fn get_bytes_length(&self, index: usize,p_index: usize) -> Vlength {
        let h = self.0[index].as_bytes().len();
        let p = self.1[p_index].as_bytes().len();
        // let p = self.1[p1_index][p2_index].as_bytes().len();

        Vlength(h, p)
    }
    pub fn get_header(&self, index: usize) -> Box<[u8]> {
        let h = &self.0[index].clone();
        let mut b_array: [u8; 100000] = [0; 100000];
        let bytes_iter = h.as_bytes().iter().cloned();

        for (i, b) in bytes_iter.enumerate() {
            b_array[i] = b;
        }

        Box::new(b_array)
    }

    pub fn get_paragraph(&self, index: usize) -> Box<[u8]> {
        let p = &self.1[index].clone();
        let mut b_array: [u8; 100000] = [0; 100000];
        let bytes_iter = p.as_bytes().iter().cloned();

        for (i, b) in bytes_iter.enumerate() {
            b_array[i] = b;
        }

        Box::new(b_array)
    }
}

#[wasm_bindgen]
pub async fn scrape_who_news() -> Vtuple {
    let url = String::from(
        "https://www.who.int/emergencies/diseases/novel-coronavirus-2019/events-as-they-happen",
    );
    let client = reqwest::Client::new();

    let res = client.get(&url).send().await.unwrap();

    let body = res.text().await.unwrap();

    let scraper = Html::parse_document(&body);

    let h_selector = Selector::parse(".sf-content-block h2").unwrap();

    let selector = Selector::parse(".sf-content-block").unwrap();

    let p_selector = Selector::parse("p").unwrap();

    let mut h_vec = vec![];
    let mut p_vec = vec![];

    for (i, c) in scraper.select(&h_selector).enumerate() {
        if i > 0 {
            for t in c.text() {
                if !t.is_empty() {
                    h_vec.push(String::from(t));
                }
            }
        }
    }
    
    for c in scraper.select(&selector) {
        for (i, _p) in c.select(&p_selector).enumerate() {
            if i > 0 {
                let mut text = c.text().take(1).next().unwrap().to_string();
                for (y, t) in c.text().enumerate() {
                    if y > 0 {
                        if !t.is_empty() {
                            if !t.to_lowercase().contains("key materials") && t != " " {
                                text.push_str(t)
                            }
                        }
                    }
                }
                p_vec.push(text);
            }
        }
    }
    Vtuple(h_vec,p_vec)
}
