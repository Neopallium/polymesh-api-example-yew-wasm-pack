use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Nav;
use crate::providers::backend::BackendContext;

use crate::pages::{Accounts, Explorer, PageNotFound, Settings, Connecting};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
  #[at("/")]
  Explorer,
  #[at("/accounts")]
  Accounts,
  #[at("/setings")]
  Settings,
  #[not_found]
  #[at("/404")]
  NotFound,
}

#[function_component]
pub fn PageRouter() -> Html {
  let backend = use_context::<BackendContext>().expect("Backend Context");
  html! {
      <BrowserRouter>
          <Nav />
  
          <main>
              if backend.is_connected() {
                  <Switch<Route> render={switch} />
              } else {
                  <Connecting />
              }
          </main>
      </BrowserRouter>
  }
}

fn switch(routes: Route) -> Html {
  match routes {
    Route::Explorer => {
      html! { <Explorer /> }
    }
    Route::Accounts => {
      html! { <Accounts /> }
    }
    Route::Settings => {
      html! { <Settings /> }
    }
    Route::NotFound => {
      html! { <PageNotFound /> }
    }
  }
}
