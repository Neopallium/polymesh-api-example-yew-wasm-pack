use core::str::FromStr;
use std::rc::Rc;
use std::collections::BTreeMap;

use yew::prelude::*;

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
  cb: Callback<Msg>,
  pub selected: String,
  pub names: Vec<String>,
  pub accounts: BTreeMap<String, AccountInfo>,
}

impl Accounts {
  pub fn new(cb: Callback<Msg>) -> Self {
    let selected: String = LocalStorage::get(SELECTED_KEY).unwrap_or_default();

    Self {
      cb,
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
    //log::info!("accounts = {:#?}", self.accounts);
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

  pub fn select_account(&self, name: String) {
    if self.selected != name {
      self.cb.emit(Msg::SelectedAccount(name));
    }
  }

  pub fn get_selected_account(&self) -> Option<&AccountInfo> {
    self.accounts.get(&self.selected)
  }

  pub fn iter(&self) -> impl Iterator<Item = &AccountInfo> {
    self.accounts.values()
  }
}

pub type AccountsContext = Rc<Accounts>;

pub enum Msg {
  BackendContextUpdated(BackendContext),
  SelectedAccount(String),
  UpdateAccountDetails(AccountInfo),
  Web3Enable(Result<Vec<web3::Extension>, String>),
  Web3Accounts(Result<Vec<web3::Account>, String>),
}

pub struct AccountsProvider {
  backend: BackendContext,
  _context_listener: ContextHandle<BackendContext>,
  accounts: Accounts,
}

impl AccountsProvider {
  fn get_accounts(&self, ctx: &Context<Self>) {
    ctx.link().send_future(async move {
      Msg::Web3Accounts(web3::accounts().await)
    });
  }

  fn update_account_details(&self, ctx: &Context<Self>) {
    if let Some(api) = self.backend.api() {
      let link = ctx.link().clone();
      let accounts = self.accounts.iter().cloned().collect::<Vec<_>>();
      wasm_bindgen_futures::spawn_local(async move {
        for mut info in accounts {
          if info.identity.is_some() {
            // Already have the details, skip.
            continue;
          }
          match info.query_account_details(&api).await {
            Ok(true) => {
              log::info!("Got account details: {info:?}");
              link.send_message(Msg::UpdateAccountDetails(info));
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
  }
}

#[derive(Properties, Debug, PartialEq)]
pub struct AccountsProviderProps {
  pub children: Children,
}

impl Component for AccountsProvider {
  type Message = Msg;
  type Properties = AccountsProviderProps;

  fn create(ctx: &Context<Self>) -> Self {
    let cb = ctx.link().callback(|m| m);
    let (backend, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::BackendContextUpdated))
            .expect("No Backend Context Provided");
    let provider = Self {
      backend,
      _context_listener,
      accounts: Accounts::new(cb),
    };
    ctx.link().send_future(async move {
      Msg::Web3Enable(web3::enable().await)
    });

    provider
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::BackendContextUpdated(backend) => {
        self.backend = backend;
        self.update_account_details(ctx);
      }
      Msg::SelectedAccount(name) => {
        self.accounts.update_selected(name);
        self.update_account_details(ctx);
      }
      Msg::UpdateAccountDetails(info) => {
        self.accounts.update_account_details(info);
      }
      Msg::Web3Enable(Ok(extensions)) => {
        log::info!("web3 extensions = {extensions:#?}");
        self.get_accounts(ctx);
      }
      Msg::Web3Accounts(Ok(accounts)) => {
        self.accounts.update_accounts(accounts);
        self.update_account_details(ctx);
      }
      Msg::Web3Enable(Err(err)) | Msg::Web3Accounts(Err(err)) => {
        log::error!("Web3 failed: {err:?}");
      }
    }
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let accounts = Rc::new(self.accounts.clone());
    html! {
      <ContextProvider<AccountsContext> context={accounts}>
        { ctx.props().children.clone()}
      </ContextProvider<AccountsContext>>
    }
  }
}
