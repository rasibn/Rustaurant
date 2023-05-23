use crate::components::home::{card::Card, card::Props as CardProps, search_input::SearchInput};
use crate::components::layout::Layout;
use gloo_net::{http::Request, Error};
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

#[function_component(Home)]
pub fn home() -> Html {
    let restaurants: UseStateHandle<Option<Restaurants>> = use_state_eq(|| None);
    {
        let restaurants = restaurants.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    //let fetched_resturants = Request::get("http://localhost:3000/restaurants/all/").send().await;
                    // fetching the resulutants with body set to true
                    let fetched_users = Request::get("https://dummyjson.com/users").send().await;

                    let response = Request::get("http://localhost:3000/restaurants/all/")
                    .send()
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await
                    .unwrap();

                    // You can see how the response looks with the following console_log then ask gpt-3 to generate the appropriate deserialized struct
                    //web_sys::console::log_1(&format!("Response: {:?}", response).into());
                    let fetched_resturants = from_value::<ApiResponse>(response).unwrap();

                    web_sys::console::log_1(&format!("Fetched resturants: {:?}", fetched_resturants).into());
                    web_sys::console::log_1(&format!("Fetched users: {:?}", fetched_users).into());
                    
                    match fetched_users {
                        Ok(response) => {
                            //web_sys::console::log_1(&format!("Response: {:?}", response).into());
                            restaurants.set(Some(Restaurants {
                                restaurants: (vec![
                                    CardProps {
                                        name: String::from("Dominos"),
                                        image: String::from("/images/dominos.jpg"),
                                        description: String::from("Dominos is a pizza restaurant"),
                                    },
                                    CardProps {
                                        name: String::from("Dominos"),
                                        image: String::from("/images/dominos.jpg"),
                                        description: String::from("Dominos is a pizza restaurant"),
                                    },
                                    CardProps {
                                        name: String::from("Dominos"),
                                        image: String::from("/images/dominos.jpg"),
                                        description: String::from("Dominos is a pizza restaurant"),
                                    },
                                    CardProps {
                                        name: String::from("Dominos"),
                                        image: String::from("/images/dominos.jpg"),
                                        description: String::from("Dominos is a pizza restaurant"),
                                    },
                                    CardProps {
                                        name: String::from("Dominos"),
                                        image: String::from("/images/dominos.jpg"),
                                        description: String::from("Dominos is a pizza restaurant"),
                                    },
                                ]),
                            }))
                        }
                        Err(e) => println!("Error: {:?}", e),
                    }
                });
            },
            (),
        );
    }

    let restaurant_list_logic = match restaurants.as_ref() {
        Some(restaurants) => restaurants
            .restaurants
            .iter()
            .map(|restaurant| {
                html! {
                  <Card name={restaurant.name.clone()}
                        image={restaurant.image.clone()}
                        description={restaurant.description.clone()} />
                }
            })
            .collect(),
        None => {
            html! {
                <div class="flex justify-center items-center h-screen">
                <div class="inline-flex space-x-4">
                    <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce"></div>
                    <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce"></div>
                    <div class="w-3 h-3 bg-blue-500 rounded-full animate-bounce"></div>
                </div>
                </div>
        }
        },
    };

    let onsubmit = Callback::from(move |search: String| {
        web_sys::console::log_1(&format!("Search ainput: {:?}", search).into());
    });

    html! {
        <Layout>
            <div>
                <div>
                    <SearchInput {onsubmit} />
                </div>
                <div class="grid grid-cols-4 gap-6 mx-16">
                { restaurant_list_logic}
                </div>
            </div>
        </Layout>
    }
}
