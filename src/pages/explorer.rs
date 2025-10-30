use leptos::*;

use crate::components::recent_blocks::RecentBlocks;
use crate::components::recent_events::RecentEvents;

#[component]
pub fn Explorer() -> impl IntoView {
  view! {
      <div class="columns">
          <RecentBlocks />
          <RecentEvents />
      </div>
  }
}
