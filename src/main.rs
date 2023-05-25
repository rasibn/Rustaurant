use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::write_a_review::UserReview;
mod pages;
mod components;
mod utils;

use pages::{
    about::About, home::Home, not_found::NotFound, restaurant::Restaurant, login::Login, create_account::CreateAccount, redirecting::Redirecting, submitting::Submitting
};


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/search")]
    SearchEmpty,
    #[at("/search/:query")]
    Search { query: String },
    #[at("/redirecting/:route")]
    Redirecting { route: String},
    #[at("/submitting/:review_hex")]
    Submitting { review_hex: String},
    #[at("/about")]
    About,
    #[at("/create_account")]
    CreateAccount,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/restaurant/:name")] //TODO: need to add a dynamic route here 
    Restaurant {name: String}

}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => html! { <About /> },
        Route::Home => html! { <Home query="" /> },
        Route::Search { query } => html! { <Home {query} /> },
        Route::SearchEmpty => html! { <Home query="" /> },
        Route::Redirecting {route} => html! { <Redirecting {route} /> },
        Route::Submitting {review_hex} => html! { <Submitting {review_hex} /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Restaurant { name }=> html! { <Restaurant {name} />},
        Route::Login => html! { <Login /> },
        Route::CreateAccount => html! { <CreateAccount /> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

