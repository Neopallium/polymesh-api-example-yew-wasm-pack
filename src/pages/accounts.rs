use yew::prelude::*;

#[function_component]
pub fn Accounts() -> Html {
  html! {
      <div class="tile is-ancestor is-vertical">
          <div class="tile is-child hero">
              <div class="hero-body container pb-0">
                  <h1 class="title is-1">{ "Accounts..." }</h1>
                  <h2 class="subtitle">{ "...to the best yew content" }</h2>
              </div>
          </div>
      </div>
  }
}
