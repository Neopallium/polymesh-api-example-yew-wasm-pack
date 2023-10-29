use std::rc::Rc;
use std::collections::BTreeMap;

use yew::prelude::*;

use crate::web3;
use crate::providers::backend::*;
use crate::providers::backend::client::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AccountInfo {
  pub name: String,
  pub address: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct Accounts {
  pub accounts: BTreeMap<String, AccountInfo>,
}

impl Accounts {
  pub fn update_accounts(&mut self, accounts: Vec<web3::Account>) {
    for account in accounts {
      let name = account.meta.name;
      let info = AccountInfo {
        address: account.address,
        name: name.clone(),
      };
      self.accounts.insert(name, info);
    }
    log::info!("accounts = {:#?}", self.accounts);
  }

  pub fn iter(&self) -> impl Iterator<Item = &AccountInfo> {
    self.accounts.values()
  }
}

pub type AccountsContext = Rc<Accounts>;

pub enum Msg {
  BackendContextUpdated(BackendContext),
  Web3Enable(Result<Vec<web3::Extension>, String>),
  Web3Accounts(Result<Vec<web3::Account>, String>),
}

pub struct AccountsProvider {
  backend: BackendContext,
  _context_listener: ContextHandle<BackendContext>,
  accounts: Accounts,
}

impl AccountsProvider {
  fn get_accounts(&mut self, ctx: &Context<Self>) {
    ctx.link().send_future(async move {
      Msg::Web3Accounts(web3::accounts().await)
    });
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
    let (backend, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::BackendContextUpdated))
            .expect("No Backend Context Provided");
    let provider = Self {
      backend,
      _context_listener,
      accounts: Accounts::default(),
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
      }
      Msg::Web3Enable(Ok(extensions)) => {
        log::info!("web3 extensions = {extensions:#?}");
        self.get_accounts(ctx);
      }
      Msg::Web3Accounts(Ok(accounts)) => {
        self.accounts.update_accounts(accounts);
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
