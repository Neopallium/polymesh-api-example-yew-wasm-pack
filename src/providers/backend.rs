use leptos::*;

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
  epoch: usize,
  url: String,
  state: BackendState,
}

impl Backend {
  pub fn new(url: String) -> Self {
    Self {
      epoch: 0,
      url,
      state: BackendState::Connecting,
    }
  }

  pub fn connect_to(&mut self, url: String, set_state: WriteSignal<BackendState>) {
    if self.url != url {
      self.url = url.clone();
      self.state = BackendState::Connecting;
      set_state.set(BackendState::Connecting);
      
      spawn_local(async move {
        match Api::new(&url).await {
          Ok(api) => {
            set_state.set(BackendState::Connected(api));
          }
          Err(err) => {
            log::error!("Failed to connect to backend: {err:?}");
          }
        }
      });
    }
  }
}

impl core::ops::Deref for Backend {
  type Target = BackendState;

  fn deref(&self) -> &Self::Target {
    &self.state
  }
}

#[component]
pub fn BackendProvider(children: Children) -> impl IntoView {
  let (state, set_state) = create_signal(BackendState::Connecting);
  let (backend, set_backend) = create_signal(Backend::new("".into()));
  
  provide_context(state);
  provide_context(set_state);
  provide_context(backend);
  provide_context(set_backend);
  
  children()
}

pub fn use_backend_state() -> (ReadSignal<BackendState>, WriteSignal<BackendState>) {
  (
    use_context::<ReadSignal<BackendState>>().expect("BackendState context"),
    use_context::<WriteSignal<BackendState>>().expect("BackendState setter context")
  )
}

pub fn use_backend() -> (ReadSignal<Backend>, WriteSignal<Backend>) {
  (
    use_context::<ReadSignal<Backend>>().expect("Backend context"),
    use_context::<WriteSignal<Backend>>().expect("Backend setter context")
  )
}
