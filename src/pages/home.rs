use yew::{function_component, html, Html};
use crate::components::layout::Layout;

#[function_component(Home)]
pub fn home() -> Html {
    html! { 
        <Layout>
            <h1>{"Welcome to my restaurant review app!"}</h1>
        </Layout>
    }
}
