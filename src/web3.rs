use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Web3Extension {
    pub name: String,
    pub version: String,
}

#[wasm_bindgen(module = "/web3.js")]
extern "C" {
    async fn enableExtensions(app: &str) -> JsValue;
}

pub async fn enable() -> Result<(), String> {
    let extensions: Vec<Web3Extension> = enableExtensions("Polymesh Yew App")
      .await
      .into_serde()
      .map_err(|e| e.to_string())?;
    log::info!("extensions = {:?}", extensions);
    Ok(())
}
