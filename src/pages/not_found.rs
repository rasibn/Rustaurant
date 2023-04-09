use yew::{function_component, html, Html};
use crate::components::layout::Layout;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! { 
        <Layout>
        <h1>{ "404" }</h1>  
        <p> {"Sorry, the page you were looking for could not be found. This may have occurred because the page has been deleted, the URL has been mistyped, or the page is temporarily unavailable. Please check the URL for any errors or try navigating back to the homepage to find the content you were looking for. If you're still unable to find what you're looking for, please contact our support team for assistance. We apologize for any inconvenience this may have caused and appreciate your patience as we work to resolve the issue. "}</p>  
        </Layout>
    }
}
