#![recursion_limit = "512"]

mod app;
mod providers;
mod components;
mod pages;
pub mod web3;

pub use app::*;
pub use providers::*;
pub use components::*;
pub use pages::*;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    wasm_bindgen_futures::spawn_local(async move {
      let extensions = web3::enable().await;
      log::info!("extensions = {:?}", extensions);
    });
    yew::Renderer::<app::App>::new().render();
    Ok(())
}
