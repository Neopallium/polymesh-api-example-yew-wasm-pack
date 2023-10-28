use yew::prelude::*;

#[function_component]
pub fn RecentEvents() -> Html {
  html! {
      <div class="column is-half">
          <table class="table is-fullwidth is-bordered">
              <thead>
                  <tr>
                      <th colspan="2"><h1>{ "Recent events" }</h1></th>
                  </tr>
              </thead>
              <tbody>
                  <tr>
                      <td>{ "Event" }</td>
                      <td>{ "12,345" }</td>
                  </tr>
              </tbody>
          </table>
      </div>
  }
}
