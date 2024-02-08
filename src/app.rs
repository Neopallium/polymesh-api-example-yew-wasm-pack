use yew::prelude::*;

use crate::providers::backend::BackendProvider;
use crate::providers::blocks::BlocksProvider;
use crate::providers::accounts::AccountsProvider;
use crate::providers::settings::SettingsProvider;
use crate::pages::PageRouter;

pub struct App {}

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {}
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    html! {
        <BackendProvider>
          <SettingsProvider>
            <AccountsProvider>
              <BlocksProvider>
                <PageRouter />
              </BlocksProvider>
            </AccountsProvider>
          </SettingsProvider>
        </BackendProvider>
    }
  }
}
