use yew::{function_component, html, Html};
use crate::components::{layout::Layout};

#[function_component(About)]
pub fn about() -> Html {

    html! { 
        <Layout>
            <h1>{"About us"}</h1>
            <p>{"This is a restaurant review app. It is a work in progress."}</p>
            //<Card/>
        </Layout> 
    }
}
