use crate::components::layout::Layout;
use yew::prelude::*;

#[function_component(Restaurant)]
pub fn restaurant() -> Html {
    html! { 
        <Layout>
        <h1>{ "Restuarant Page" }</h1>  
        <p class={classes!("dark:text-gray-400")}> {"Sorry, the page you were looking for could not be found. This may have occurred because the page has been deleted, the URL has been mistyped, or the page is temporarily unavailable. Please check the URL for any errors or try navigating back to the homepage to find the content you were looking for. If you're still unable to find what you're looking for, please contact our support team for assistance. We apologize for any inconvenience this may have caused and appreciate your patience as we work to resolve the issue. "}</p>  
        </Layout>
    }
}
