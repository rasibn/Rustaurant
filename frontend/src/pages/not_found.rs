use crate::components::layout::Layout;
use yew::prelude::*;
use crate::Route;
use yew_router::prelude::Link;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <Layout>
            <div class="flex flex-col items-center justify-center min-h-screen">
                <div class="max-w-3xl mx-auto text-white text-center">
                    <h1 class="text-5xl font-bold mb-8">{"404"}</h1>
                    <p class="text-gray-300 mb-8">{"Sorry, the page you were looking for could not be found. This may have occurred because the page has been deleted, the URL has been mistyped, or the page is temporarily unavailable. Please check the URL for any errors or try navigating back to the homepage to find the content you were looking for. If you're still unable to find what you're looking for, please contact our support team for assistance. We apologize for any inconvenience this may have caused and appreciate your patience as we work to resolve the issue."}</p>

                    <Link<Route> to={Route::Home} classes="bg-blue-500 text-white font-semibold px-4 py-2 rounded hover:bg-blue-600">{"Go Back Home"}</Link<Route>>
                </div>
            </div>
        </Layout>
    }
}
