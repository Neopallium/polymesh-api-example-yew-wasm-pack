use yew::prelude::*;

use gloo_storage::{LocalStorage, Storage};

use serde::{Deserialize, Serialize};

use crate::providers::backend::BackendProvider;
use crate::providers::blocks::BlocksProvider;
use crate::providers::accounts::AccountsProvider;
use crate::pages::PageRouter;

const APP_KEY: &str = "example.app.polymesh.network";

#[derive(Debug, Serialize, Deserialize)]
pub struct AppSettings {
  pub url: String,
}

impl Default for AppSettings {
  fn default() -> Self {
    Self {
      url: "ws://localhost:9944".into(),
    }
  }
}

pub struct App {
  _settings: AppSettings,
}

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let settings: AppSettings = LocalStorage::get(APP_KEY).unwrap_or_default();
    Self {
      _settings: settings,
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
        <BackendProvider>
          <AccountsProvider>
            <BlocksProvider>
              <PageRouter />
            </BlocksProvider>
          </AccountsProvider>
        </BackendProvider>
    }
  }
}
