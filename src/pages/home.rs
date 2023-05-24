use crate::components::home::{card::Card, card::Props as CardProps, search_input::SearchInput};
use crate::components::layout::Layout;
use gloo_net::{http::Request};
use serde::Deserialize;
use yew::{
    function_component, html, use_effect_with_deps, use_state_eq, Callback, Html, Properties,
    UseStateHandle,
};
use serde_json::from_value;

#[derive(Properties, Deserialize, PartialEq, Clone)]
pub struct Restaurants {
    pub restaurants: Vec<CardProps>,
}

#[derive(Properties, Deserialize, PartialEq, Clone, Debug)]
pub struct Restaurant {
    pub name: String,
    pub description: String,
    pub num_star: [i32; 5],
}

#[derive(serde::Deserialize, Debug)]
struct ApiResponse {
    data: Vec<Restaurant>,
    success: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub query: String,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    let restaurants: UseStateHandle<Option<Restaurants>> = use_state_eq(|| None);

    use_effect_with_deps(
        {
            let query = props.query.clone();
            let restaurants = restaurants.clone();
            move |_| {
                fetch_restaurants(query, restaurants);
            }
        },
        (), // Empty vector as the second argument
    );

    let restaurant_card_list = match restaurants.as_ref() {
        Some(restaurants) if !restaurants.restaurants.is_empty() => {
            html! {
                <div>
                    <div class="grid grid-cols-4 gap-6 mx-16">
                        {restaurants.restaurants.iter().map(|restaurant| {
                            html! {
                                <Card
                                    name={restaurant.name.clone()}
                                    description={restaurant.description.clone()}
                                />
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        }
        Some(_) => {
            html! {
                <div class="flex justify-center items-center h-screen">
                    <div class="inline-flex space-x-4">
                        <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce"></div>
                        <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce"></div>
                        <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce"></div>
                    </div>
                </div>
            }
        }
        None => html! {},
    };

    let onsubmit = Callback::from(move |search: String| {
        web_sys::console::log_1(&format!("Search input: {:?}", search).into());
    });

    html! {
        <Layout>
            <div>
                <div>
                    <SearchInput {onsubmit} />
                </div>
                { render_heading(props.query.clone()) }
                {restaurant_card_list}
            </div>
        </Layout>
    }
}

fn render_heading(query: String) -> Html {
    let heading_text = if query.is_empty() {
        "Showing All Restaurants".to_string()
    } else {
        format!("Search result for \"{}\"", query)
    };

    html! {
        <h2 class="text-2xl font-bold mb-4 text-center leading-tight text-primary">
            { heading_text }
        </h2>
    }
}

fn fetch_restaurants(query: String, restaurants: UseStateHandle<Option<Restaurants>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::get(format!("http://localhost:3000/restaurants/all/{}/", query).as_str())
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();

        let fetched_restaurants = from_value::<ApiResponse>(response).unwrap();

        if fetched_restaurants.success == false {
            restaurants.set(Some(Restaurants {
                restaurants: vec![],
            }));
            return;
        }

        restaurants.set(Some(Restaurants {
            restaurants: fetched_restaurants
                .data
                .iter()
                .map(|restaurant| CardProps {
                    name: restaurant.name.clone(),
                    description: restaurant.description.clone(),
                })
                .collect(),
        }));
    });
}
