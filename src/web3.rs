use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::from_value;

use serde::{Serialize, Deserialize};

#[wasm_bindgen(module = "/web3.js")]
extern "C" {
  #[wasm_bindgen(catch, js_name = enableExtensions)]
  async fn web3_enable_extensions(app: &str) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(catch, js_name = getAccounts)]
  async fn web3_get_accounts() -> Result<JsValue, JsValue>;

  #[wasm_bindgen(catch, js_name = signPayload)]
  async fn web3_sign_payload(payload: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
  pub address: String,
  pub genesis_hash: Option<String>,
  pub name: Option<String>,
  #[serde(rename = "type")]
  pub key_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountMeta {
  pub genesis_hash: Option<String>,
  #[serde(default)]
  pub name: String,
  pub source: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountWithMeta {
  pub address: String,
  pub meta: AccountMeta,
  pub key_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extension {
  pub name: String,
  pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
  pub address: String,
  #[serde(rename = "blockHash")]
  pub block_hash: String,
  #[serde(rename = "blockNumber")]
  pub block_number: String,
  pub era: String,
  #[serde(rename = "genesisHash")]
  pub genesis_hash: String,
  pub method: String,
  pub nonce: String,
  #[serde(rename = "specVersion")]
  pub spec_version: String,
  pub tip: String,
  #[serde(rename = "transactionVersion")]
  pub transaction_version: String,
  #[serde(rename = "signedExtensions")]
  pub signed_extensions: Vec<String>,
  pub version: u32,
}

impl Payload {
  pub fn new(address: String, method: String) -> Self {
    Self {
      address,
      method,
      block_hash: "0xd4d4069f6842e848e3cd37f2a7b70365edb1f2ea67d6cf9d10ce8fea9d56cea8".to_string(),
      block_number: "0x00000000".to_string(),
      era: "0x0000".to_string(),
      genesis_hash: "0xd4d4069f6842e848e3cd37f2a7b70365edb1f2ea67d6cf9d10ce8fea9d56cea8".to_string(),
      nonce: "0x00000000".to_string(),
      tip: "0x00000000000000000000000000000000".to_string(),
      spec_version: "0x005B8D84".to_string(),
      transaction_version: "0x00000004".to_string(),
      signed_extensions: vec![
        "CheckSpecVersion".to_string(),
        "CheckTxVersion".to_string(),
        "CheckGenesis".to_string(),
        "CheckMortality".to_string(),
        "CheckNonce".to_string(),
        "CheckWeight".to_string(),
        "ChargeTransactionPayment".to_string(),
        "StoreCallMetadata".to_string(),
      ],
      version: 4,
    }
  }
}

fn fmt_err(err: JsValue) -> String {
  format!("{:?}", err)
}

pub async fn enable() -> Result<Vec<Extension>, String> {
  let list = web3_enable_extensions("Polymesh Yew App").await
    .map_err(fmt_err)?;
  let extensions: Vec<Extension> = from_value(list)
    .map_err(|e| e.to_string())?;
  log::info!("extensions = {:?}", extensions);
  let _accounts = accounts().await?;
  Ok(extensions)
}

pub async fn accounts() -> Result<Vec<AccountWithMeta>, String> {
  let list = web3_get_accounts().await
    .map_err(fmt_err)?;
  let accounts: Vec<AccountWithMeta> = from_value(list)
    .map_err(|e| e.to_string())?;
  log::info!("accounts = {:#?}", accounts);
  //let payload = Payload::new(accounts[0].address.clone(), "0x000000".to_string());
  //let _sig = sign_payload(payload).await?;
  Ok(accounts)
}

pub async fn sign_payload(payload: Payload) -> Result<JsValue, String> {
  let payload = serde_wasm_bindgen::to_value(&payload)
    .map_err(|e| e.to_string())?;
  let sig = web3_sign_payload(payload).await
    .map_err(fmt_err)?;
  Ok(sig)
}
