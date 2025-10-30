use leptos::*;

use crate::providers::backend::BackendProvider;
use crate::providers::blocks::BlocksProvider;
use crate::providers::accounts::AccountsProvider;
use crate::providers::settings::SettingsProvider;
use crate::pages::PageRouter;

#[component]
pub fn App() -> impl IntoView {
    view! {
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
