use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! { <h1>{ "404" }</h1> }
}
