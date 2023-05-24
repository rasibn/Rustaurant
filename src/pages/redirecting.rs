use crate::components::layout::Layout;
use yew::prelude::*;
use crate::Route;
use yew_router::prelude::Link;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub route: String,
}

#[function_component(Redirecting)]
pub fn redirecting(props: &Props) -> Html {
    let window = window().expect("Failed to retrieve window object");
    let route: Vec<&str> = props.route.split("+").collect();
    let redirect_url = format!("/{}/{}", route[0], route[1]);

    window
        .location()
        .set_href(&redirect_url)
        .expect("Failed to redirect");

    html! {
        <Layout>
            <div class="bg-gray-100 flex flex-col items-center justify-center min-h-screen">
                <div class="max-w-3xl mx-auto">
                    <h1 class="text-5xl font-bold mb-8">{"Redirecting..."}</h1>
                    <p class="text-gray-500 mb-8">{"You will be redirected to the homepage shortly."}</p>
                    <Link<Route> to={Route::Home} classes="bg-blue-500 text-white font-semibold px-4 py-2 rounded hover:bg-blue-600">{"Go Back Home"}</Link<Route>>
                </div>
            </div>
        </Layout>
    }
}
