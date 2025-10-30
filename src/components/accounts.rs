use leptos::*;

use crate::providers::accounts::{use_accounts, AccountInfo};

#[component]
pub fn Accounts() -> impl IntoView {
  let (accounts, _) = use_accounts();

  view! {
    <div class="column is-half">
        <table class="table is-fullwidth is-bordered">
            <thead>
                <tr>
                    <th colspan="4"><h1>{ "Accounts" }</h1></th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    accounts.get().iter().map(|acc| view_account(acc)).collect_view()
                }}
            </tbody>
        </table>
    </div>
  }
}

fn view_account(item: &AccountInfo) -> impl IntoView {
  let name = item.name.clone();
  let address = item.address();
  let identity = item.identity();
  
  view! {
      <tr>
          <th>{ name }</th>
          <td>{ address }</td>
          <td>{ identity }</td>
      </tr>
  }
}
