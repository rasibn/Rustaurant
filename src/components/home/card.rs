use crate::Route;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::Link;
use serde::Deserialize;

#[derive(Properties, Deserialize, PartialEq, Clone, Debug)]
pub struct Props {
    pub name: String,
    pub description: String,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let max_description_length = 100;
    let trimmed_description = if props.description.chars().count() > max_description_length {
        let trimmed = props.description.chars().take(max_description_length).collect::<String>();
        format!("{}...", trimmed)
    } else {
        props.description.clone()
    };

    html! {
        <Link<Route> to={Route::Restaurant { name: props.name.clone() }}>
            <div class="max-w-xs min-h-40 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
                <img class="rounded-t-lg" src={format!("/images/{}.jpg", props.name.clone())} alt={format!("Image for {}", &props.name)}/>
                <div class="p-5">
                    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{ &props.name }</h5>
                    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{ &trimmed_description }</p>
                    { render_read_more_button(&props.description) }
                </div>
            </div>
        </Link<Route>>
    }
}

fn render_read_more_button(description: &str) -> Html {
    let max_description_length = 100;
    if description.chars().count() > max_description_length {
        html! {
            <button class="text-sm font-medium text-blue-600 hover:underline dark:text-blue-500">
                {"Read more"}
            </button>
        }
    } else {
        html! {}
    }
}
