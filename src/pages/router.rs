use leptos::*;
use leptos_router::*;

use crate::components::Nav;
use crate::providers::backend::use_backend_state;

use crate::pages::{AccountsPage, Connecting, Explorer, PageNotFound, Settings};

#[derive(Clone, PartialEq)]
enum Route {
    Explorer,
    Accounts,
    Settings,
    NotFound,
}

#[component]
pub fn PageRouter() -> impl IntoView {
    let (backend_state, _) = use_backend_state();

    view! {
        <Router>
            <Nav />
            <main>
                {move || {
                    if backend_state.get().is_connected() {
                        view! {
                            <Routes>
                                <Route path="/" view=Explorer />
                                <Route path="/accounts" view=AccountsPage />
                                <Route path="/settings" view=Settings />
                                <Route path="/*any" view=PageNotFound />
                            </Routes>
                        }.into_view()
                    } else {
                        view! { <Connecting /> }.into_view()
                    }
                }}
            </main>
        </Router>
    }
}
