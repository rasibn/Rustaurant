use yew::prelude::*;
use yew_router::prelude::*;


//import functional components from yew

mod pages;
mod components;

use pages::{
    about::About, home::Home, not_found::NotFound, restaurant::Restaurant
};


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/restaurant/:name")] //TODO: need to add a dynamic route here 
    Restaurant

}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => html! { <About /> },
        Route::Home => html! { <Home /> },
        Route::Secure => html! { <Secure /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Restaurant => html! { <Restaurant />}
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


#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}

