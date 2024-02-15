use std::rc::Rc;

use yew::prelude::*;

use serde::{Deserialize, Serialize};

use gloo_storage::{LocalStorage, Storage};

use crate::providers::backend::*;

const APP_KEY: &str = "example.app.polymesh.network";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl AppSettings {
  pub fn update_settings(&mut self, settings: Self) {
    *self = settings;
    log::info!("app settings = {:#?}", self);
  }
}

pub type SettingsContext = Rc<AppSettings>;

pub enum Msg {
  BackendContextUpdated(BackendContext),
}

pub struct SettingsProvider {
  backend: BackendContext,
  _context_listener: ContextHandle<BackendContext>,
  settings: AppSettings,
}

#[derive(Properties, Debug, PartialEq)]
pub struct SettingsProviderProps {
  pub children: Children,
}

impl Component for SettingsProvider {
  type Message = Msg;
  type Properties = SettingsProviderProps;

  fn create(ctx: &Context<Self>) -> Self {
    let settings: AppSettings = LocalStorage::get(APP_KEY).unwrap_or_else(|_| {
      let settings = AppSettings::default();
      // Save settings.
      if let Err(err) = LocalStorage::set(APP_KEY, &settings) {
        log::error!("Failed to save settings: {err:?}");
      }

      settings
    });

    let (backend, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::BackendContextUpdated))
            .expect("No Backend Context Provided");
    // Set backend URL.
    backend.connect_to(settings.url.clone());

    let provider = Self {
      backend,
      _context_listener,
      settings,
    };

    provider
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::BackendContextUpdated(backend) => {
        backend.connect_to(self.settings.url.clone());
        self.backend = backend;
      }
    }
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let settings = Rc::new(self.settings.clone());
    html! {
      <ContextProvider<SettingsContext> context={settings}>
        { ctx.props().children.clone()}
      </ContextProvider<SettingsContext>>
    }
  }
}
