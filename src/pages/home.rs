use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! { <h1>{"Welcome to my restaurant review app!"}</h1> }
}
