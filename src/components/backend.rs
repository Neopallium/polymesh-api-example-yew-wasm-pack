use std::rc::Rc;

use yew::prelude::*;

pub use polymesh_api::*;

#[derive(Clone)]
pub enum BackendAction {
  ConnectTo(String),
  Connected(String, Api),
  BackendError(String),
}

#[derive(Clone)]
pub enum BackendState {
  Connecting,
  Connected(Api),
}

impl BackendState {
  pub fn api(&self) -> Option<Api> {
    match self {
      Self::Connected(api) => Some(api.clone()),
      _ => None
    }
  }

  pub fn is_connected(&self) -> bool {
    match self {
      Self::Connected(_) => true,
      _ => false,
    }
  }
}

impl PartialEq for BackendState {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Connected(_), Self::Connected(_)) => true,
      (Self::Connecting, Self::Connecting) => true,
      _ => false
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct Backend {
  cb: Callback<BackendAction>,
  epoch: usize,
  url: String,
  state: BackendState,
}

impl Backend {
  pub fn new(cb: Callback<BackendAction>, url: String) -> Self {
    Self {
      cb,
      epoch: 0,
      url,
      state: BackendState::Connecting,
    }
  }

  pub fn connect_to(&self, url: String) {
    if self.url != url {
      self.cb.emit(BackendAction::ConnectTo(url));
    }
  }

  fn update(&mut self, action: BackendAction) {
    self.epoch += 1;
    match action {
      BackendAction::ConnectTo(url) => {
        self.url = url;
        self.state = BackendState::Connecting;
      }
      BackendAction::Connected(url, api) => {
        self.url = url;
        self.state = BackendState::Connected(api);
      }
      _ => {
      }
    }
  }
}

impl core::ops::Deref for Backend {
  type Target = BackendState;

  fn deref(&self) -> &Self::Target {
    &self.state
  }
}

pub type BackendContext = Rc<Backend>;

pub struct BackendProvider {
  backend: Backend,
}

#[derive(Properties, Debug, PartialEq)]
pub struct BackendProviderProps {
  pub children: Children,
}

impl Component for BackendProvider {
  type Message = BackendAction;
  type Properties = BackendProviderProps;

  fn create(ctx: &Context<Self>) -> Self {
    let url = "ws://localhost:9944".to_string();
    let cb = ctx.link().callback(|m| m);
    cb.emit(BackendAction::ConnectTo(url));
    Self {
      backend: Backend::new(cb, "".into()),
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match &msg {
      BackendAction::ConnectTo(url) => {
        let url = url.clone();
        log::info!("Backend connect to: {url:?}");
        ctx.link().send_future(async move {
          match Api::new(&url).await {
            Ok(api) => BackendAction::Connected(url, api),
            Err(err) => {
              BackendAction::BackendError(err.to_string())
            }
          }
        });
      }
      BackendAction::BackendError(err) => {
        log::error!("Failed to connect to backend: {err:?}");
      }
      _ => (),
    }
    self.backend.update(msg);
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let backend = Rc::new(self.backend.clone());
    html! {
      <ContextProvider<BackendContext> context={backend}>
        { ctx.props().children.clone()}
      </ContextProvider<BackendContext>>
    }
  }
}
