use crate::Route;
use yew::{function_component, html, Callback, MouseEvent, Html, Properties};
use yew_router::prelude::Link;
use serde::Deserialize;
use yew::functional::use_state;

#[derive(Properties, Deserialize, PartialEq, Clone, Debug)]
pub struct Props {
    pub name: String,
    pub description: String,
}

#[function_component(BigCard)]
pub fn big_card(props: &Props) -> Html {
    let show_full_description = use_state(|| false);

    let toggle_description = {
        let show_full_description = show_full_description.clone();
        Callback::from(move |_e: MouseEvent| {
            show_full_description.set(!*show_full_description);
        })
    };

    let description = if *show_full_description || props.description.len() <= 100 {
        &props.description
    } else {
        &props.description[..200]
    };

    html! {
        <Link<Route> to={Route::Restaurant { name: props.name.clone() }}>
            <div class="max-w-xs mx-auto bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800" style="width: 320px; height: 580px;">
                <img class="rounded-t-lg text-primary" src={format!("/images/{}.jpg",props.name.clone())} alt={format!("Image for {}", &props.name)}/>
                <div class="p-5 h-3/4">
                    <h5 class="mb-2 text-2xl font-bold tracking-tight text-primary">{ &props.name }</h5>
                    <p class="mb-3 font-normal text-primary">
                        { description }
                        { render_read_more_button(*show_full_description, toggle_description, props.description.len() > 100) }
                    </p>
                </div>
            </div>
        </Link<Route>>
    }
}

fn render_read_more_button(show_full_description: bool, onclick: Callback<MouseEvent>, is_truncated: bool) -> Html {
    if is_truncated {
        if show_full_description {
            html! {
                <button class="text-sm font-medium text-blue-600 hover:underline dark:text-blue-500" onclick={onclick}>
                    {"Read less"}
                </button>
            }
        } else {
            html! {
                <button class="text-sm font-medium text-blue-600 hover:underline dark:text-blue-500" onclick={onclick}>
                    {"Read more"}
                </button>
            }
        }
    } else {
        html! {}
    }
}
