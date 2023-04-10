use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {

  html! {
    <nav>
    // Navigation links go here
    <div class={classes!("text-navy-700")}>{"Navbar"}</div>
    </nav>
  }
}