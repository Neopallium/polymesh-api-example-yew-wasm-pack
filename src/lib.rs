#![recursion_limit = "512"]

mod app;
pub mod providers;
pub mod components;
pub mod pages;
pub mod web3;

pub use app::*;

use wasm_bindgen::prelude::*;
use leptos::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| view! { <App /> });
    Ok(())
}
