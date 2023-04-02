use yew::prelude::*;
use yew_router::prelude::*;



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

#[function_component(Home)]
pub fn home() -> Html {
    html! { <h1>{"Welcome to my restaurant review app!"}</h1> }
}

#[function_component(About)]
pub fn about() -> Html {
    html! { <h1>{"About us"}</h1> }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => html! { <About /> },
        Route::Home => html! { <Home />},
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
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