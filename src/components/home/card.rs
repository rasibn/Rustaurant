use crate::Route;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::Link;
use serde::Deserialize;

#[derive(Properties, Deserialize, PartialEq, Clone, Debug)]
pub struct Props {
    pub name: String,
    pub image: String,
    pub description: String,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    html! {

        <Link<Route> to={Route::Restaurant {name:props.name.clone()}}>
        <div class="max-w-xs min-h-40 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
        <a href="#">
            <img class="rounded-t-lg" src={props.image.clone()} alt={format!("Image for {}", &props.name)}/>
        </a>
        <div class="p-5">
            <a href="#">
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{ &props.name }</h5>
            </a>
            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{ &props.description }</p>
        </div>
    </div>
        </Link<Route>>


        }
}
