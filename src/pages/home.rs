use crate::components::home::{card::Card, card::Props as CardProps, search_input::SearchInput};
use crate::components::layout::Layout;
use gloo_net::{http::Request, Error};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use yew::{
    function_component, html, use_effect_with_deps, use_state_eq, Callback, Html, Properties,
    UseStateHandle,
};

#[derive(Properties, Deserialize, PartialEq, Clone)]
pub struct Resuturants {
    pub resuturants: Vec<CardProps>,
}

#[function_component(Home)]
pub fn home() -> Html {
    let resuturants: UseStateHandle<Option<Resuturants>> = use_state_eq(|| None);

    {
        let resuturants = resuturants.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users = Request::get("https://dummyjson.com/users").send().await;
                    match fetched_users {
                        Ok(response) => {
                            web_sys::console::log_1(&format!("Response: {:?}", response).into());
                            resuturants.set(Some(Resuturants {
                                resuturants: (vec![
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

    let resturant_list_logic = match resuturants.as_ref() {
        Some(resuturants) => resuturants
            .resuturants
            .iter()
            .map(|resturant| {
                html! {
                  <Card name={resturant.name.clone()} image={resturant.image.clone()} description={resturant.description.clone()} />
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
                { resturant_list_logic}
                </div>
            </div>
        </Layout>
    }
}
