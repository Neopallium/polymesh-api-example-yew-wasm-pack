use leptos::*;

#[component]
pub fn Settings() -> impl IntoView {
  view! {
      <div class="tile is-ancestor is-vertical">
          <div class="tile is-child hero">
              <div class="hero-body container pb-0">
                  <h1 class="title is-1">{ "Settings page..." }</h1>
              </div>
          </div>
      </div>
  }
}
