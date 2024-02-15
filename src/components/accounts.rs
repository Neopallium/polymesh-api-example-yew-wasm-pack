use yew::prelude::*;

use crate::providers::accounts::{AccountsContext, AccountInfo};

#[function_component]
pub fn Accounts() -> Html {
  let accounts = use_context::<AccountsContext>().expect("Accounts context provided");

  html! {
    <div class="column is-half">
        <table class="table is-fullwidth is-bordered">
            <thead>
                <tr>
                    <th colspan="4"><h1>{ "Accounts" }</h1></th>
                </tr>
            </thead>
            <tbody>
                { for accounts.iter().map(|acc| view_account(acc)) }
            </tbody>
        </table>
    </div>
  }
}

fn view_account(item: &AccountInfo) -> Html {
  html! {
      <tr key={ item.name.clone() }>
          <th>{ &item.name }</th>
          <td>{ &item.address() }</td>
          <td>{ &item.identity() }</td>
      </tr>
  }
}
