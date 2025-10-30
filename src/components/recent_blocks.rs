use leptos::*;

use crate::providers::blocks::{use_blocks, BlockItem};

#[component]
pub fn RecentBlocks() -> impl IntoView {
  let blocks = use_blocks();

  view! {
    <div class="column is-half">
        <table class="table is-fullwidth is-bordered">
            <thead>
                <tr>
                    <th colspan="3"><h1>{ "Recent blocks" }</h1></th>
                </tr>
            </thead>
            <tbody>
                {move || {
                    let recent: Vec<_> = blocks.get().iter().map(|(_, b)| b.clone()).collect();
                    recent.into_iter().rev().map(|b| view_block_item(&b)).collect_view()
                }}
            </tbody>
        </table>
    </div>
  }
}

fn view_block_item(item: &BlockItem) -> impl IntoView {
  let number = item.number;
  let hash = item.hash.to_string();
  view! {
      <tr>
          <th>{ number }</th>
          <td>{ hash }</td>
          <td>{ "Alice" }</td>
      </tr>
  }
}
