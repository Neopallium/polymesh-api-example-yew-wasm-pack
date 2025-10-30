use leptos::*;
use leptos_router::*;

#[component]
pub fn Nav() -> impl IntoView {
  let (navbar_active, set_navbar_active) = create_signal(false);

  let toggle_navbar = move |_| {
    set_navbar_active.update(|active| *active = !*active);
  };

  view! {
      <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
          <div class="navbar-brand">
              <h1 class="navbar-item is-size-3">{ "Polymesh app" }</h1>

              <button 
                  class=move || if navbar_active.get() { "navbar-burger burger is-active" } else { "navbar-burger burger" }
                  aria-label="menu" 
                  aria-expanded="false"
                  on:click=toggle_navbar
              >
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
              </button>
          </div>
          <div class=move || if navbar_active.get() { "navbar-menu is-active" } else { "navbar-menu" }>
              <div class="navbar-start">
                  <A class="navbar-item" href="/accounts">
                      { "Accounts" }
                  </A>

                  <div class="navbar-item has-dropdown is-hoverable">
                      <div class="navbar-link">
                          { "Network" }
                      </div>
                      <div class="navbar-dropdown">
                          <A class="navbar-item" href="/">
                              { "Explorer" }
                          </A>
                      </div>
                  </div>

                  <A class="navbar-item" href="/settings">
                      { "Settings" }
                  </A>
              </div>

              <div class="navbar-end">
                  <div class="navbar-item">
                  </div>
              </div>
          </div>
      </nav>
  }
}
