use yew::prelude::*;

use crate::components::blocks::{BlocksContext, BlockItem};

#[function_component]
pub fn RecentBlocks() -> Html {
  let blocks = use_context::<BlocksContext>().expect("Blocks context provided");

  let recent: Vec<_> = blocks.iter().map(|(_, b)| b).collect();
  html! {
    <div class="column is-half">
        <table class="table is-fullwidth is-bordered">
            <thead>
                <tr>
                    <th colspan="3"><h1>{ "Recent blocks" }</h1></th>
                </tr>
            </thead>
            <tbody>
                { for recent.into_iter().rev().map(|b| view_block_item(b)) }
            </tbody>
        </table>
    </div>
  }
}

fn view_block_item(item: &BlockItem) -> Html {
  html! {
      <tr key={ item.number.to_string() }>
          <th>{ item.number }</th>
          <td>{ item.hash.to_string() }</td>
          <td>{ "Alice" }</td>
      </tr>
  }
}
