use core::str::FromStr;
use std::collections::BTreeMap;

use leptos::*;

use gloo_storage::{LocalStorage, Storage};

use polymesh_api::polymesh::types::{
  polymesh_primitives::secondary_key::KeyRecord,
};

use crate::web3;
use crate::providers::backend::*;
use crate::providers::backend::client::*;

const SELECTED_KEY: &str = "example.app.polymesh.network.selected.account";

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountInfo {
  pub name: String,
  pub account_id: AccountId,
  pub identity: Option<IdentityId>,
  pub key_type: String,
}

impl AccountInfo {
  async fn query_account_details(&mut self, api: &Api) -> Result<bool, String> {
    let rec = api.query().identity().key_records(self.account_id).await
      .map_err(|e| e.to_string())?;
    Ok(match rec {
      Some(KeyRecord::PrimaryKey(did)) => {
        self.identity = Some(did);
        self.key_type = "Primary".to_string();
        true
      }
      Some(KeyRecord::SecondaryKey(did)) => {
        self.identity = Some(did);
        self.key_type = "Secondary".to_string();
        true
      }
      Some(KeyRecord::MultiSigSignerKey(_)) => {
        false
      }
      None => {
        false
      }
    })
  }

  pub fn address(&self) -> String {
    self.account_id.to_string()
  }

  pub fn identity(&self) -> String {
    match self.identity {
      Some(did) => did.to_string(),
      None => "No identity".to_string(),
    }
  }
}

#[derive(Clone, Default, PartialEq)]
pub struct Accounts {
  pub selected: String,
  pub names: Vec<String>,
  pub accounts: BTreeMap<String, AccountInfo>,
}

impl Accounts {
  pub fn new() -> Self {
    let selected: String = LocalStorage::get(SELECTED_KEY).unwrap_or_default();

    Self {
      selected,
      names: Default::default(),
      accounts: Default::default(),
    }
  }

  pub fn update_accounts(&mut self, accounts: Vec<web3::Account>) {
    for account in accounts {
      let name = account.meta.name;
      match AccountId::from_str(&account.address) {
        Ok(account_id) => {
          let info = AccountInfo {
            account_id,
            name: name.clone(),
            identity: None,
            key_type: "".to_string(),
          };
          self.names.push(name.clone());
          self.accounts.insert(name, info);
        }
        Err(err) => {
          log::error!("Invalid account[{name}]: {err:?}");
        }
      }
    }
    // If no account is selected or missing selected account, try selecting the first.
    if self.selected == "" || !self.accounts.contains_key(&self.selected) {
      let first = self.accounts.first_key_value().map(|(name, _)| name.to_string());
      log::info!("select first account = {first:#?}");
      if let Some(name) = first {
        self.update_selected(name);
      }
    }
  }

  fn update_selected(&mut self, name: String) {
    log::info!("select account = {name:#?}");
    if self.accounts.contains_key(&name) {
      // Save selected account
      if let Err(err) = LocalStorage::set(SELECTED_KEY, &name) {
        log::error!("Failed to save selected account: {err:?}");
      }
      self.selected = name;
    }
  }

  fn update_account_details(&mut self, info: AccountInfo) {
    let account = self.accounts.entry(info.name.clone())
      .or_default();
    *account = info;
  }

  pub fn get_selected_account(&self) -> Option<&AccountInfo> {
    self.accounts.get(&self.selected)
  }

  pub fn iter(&self) -> impl Iterator<Item = &AccountInfo> {
    self.accounts.values()
  }
}

#[component]
pub fn AccountsProvider(children: Children) -> impl IntoView {
  let (accounts, set_accounts) = create_signal(Accounts::new());
  let (backend_state, _) = use_backend_state();
  
  // Enable web3 extensions
  create_effect(move |_| {
    spawn_local(async move {
      match web3::enable().await {
        Ok(extensions) => {
          log::info!("web3 extensions = {extensions:#?}");
          match web3::accounts().await {
            Ok(web3_accounts) => {
              set_accounts.update(|acc| acc.update_accounts(web3_accounts));
            }
            Err(err) => {
              log::error!("Web3 accounts failed: {err:?}");
            }
          }
        }
        Err(err) => {
          log::error!("Web3 enable failed: {err:?}");
        }
      }
    });
  });
  
  // Query account details when backend is connected
  create_effect(move |_| {
    if let BackendState::Connected(api) = backend_state.get() {
      let account_list = accounts.get().accounts.values().cloned().collect::<Vec<_>>();
      spawn_local(async move {
        for mut info in account_list {
          if info.identity.is_some() {
            continue;
          }
          match info.query_account_details(&api).await {
            Ok(true) => {
              log::info!("Got account details: {info:?}");
              set_accounts.update(|acc| acc.update_account_details(info));
            }
            Ok(false) => {
              log::trace!("account doesn't have an identity");
            }
            Err(err) => {
              log::error!("failed to query account details: {err:?}");
            }
          }
        }
      });
    }
  });

  provide_context(accounts);
  provide_context(set_accounts);
  
  children()
}

pub fn use_accounts() -> (ReadSignal<Accounts>, WriteSignal<Accounts>) {
  (
    use_context::<ReadSignal<Accounts>>().expect("Accounts context"),
    use_context::<WriteSignal<Accounts>>().expect("Accounts setter context")
  )
}
