use leptos::*;

use crate::components::accounts::Accounts;

#[component]
pub fn AccountsPage() -> impl IntoView {
  view! {
      <div class="columns">
          <Accounts />
      </div>
  }
}
