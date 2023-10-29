use yew::prelude::*;

use crate::components::accounts::Accounts;

#[function_component]
pub fn AccountsPage() -> Html {
  html! {
      <div class="columns">
          <Accounts />
      </div>
  }
}
