use yew::prelude::*;

use crate::components::recent_blocks::RecentBlocks;
use crate::components::recent_events::RecentEvents;

#[function_component]
pub fn Explorer() -> Html {
  html! {
      <div class="columns">
          <RecentBlocks />
          <RecentEvents />
      </div>
  }
}
