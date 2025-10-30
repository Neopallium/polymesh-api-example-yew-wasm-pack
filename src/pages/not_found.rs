use leptos::*;

#[component]
pub fn PageNotFound() -> impl IntoView {
  view! {
      <div class="tile is-ancestor is-vertical">
          <div class="tile is-child hero">
              <div class="hero-body container pb-0">
                  <h1 class="title is-1">{ "Page not found" }</h1>
              </div>
          </div>
      </div>
  }
}
