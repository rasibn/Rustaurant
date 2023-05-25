use crate::components::big_card::BigCard;
use crate::components::layout::Layout;
use crate::components::rating::Rating;
use crate::components::review::Review;
use crate::components::write_a_review::WriteAReview;
use gloo_net::http::Request;
use serde::Deserialize;
use crate::components::write_a_review::UserReview;
use yew::prelude::*;
use serde_json::from_value;
use std::rc::Rc;

#[derive(Deserialize, Debug)]
struct ApiResponseForInfo {
    data: Vec<RestaurantInfo>,
    success: bool,
}

#[derive(Deserialize, Debug)]
struct ApiResponseForReviews {
    data: Vec<UserReview>,
    success: bool,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
struct RestaurantInfo {
    name: String,
    description: String,
    num_star: [i32; 5],
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(Restaurant)]
pub fn restaurant(props: &Props) -> Html {
    let user_reviews = use_state_eq(|| None::<Vec<UserReview>>);
    let restaurant_info = use_state_eq(|| Some(Rc::new(RestaurantInfo {
        name: String::from("Loading..."),
        description: String::from("Loading..."),
        num_star: [0; 5],
    })));

    use_effect_with_deps(
        {
            let name = props.name.clone();
            let user_reviews: UseStateHandle<Option<Vec<UserReview>>> = user_reviews.clone();
            let restaurant_info = restaurant_info.clone();
            move |_| {
                fetch_restuarant_data_async(name, user_reviews, restaurant_info);
            }
        },
        (), // Empty vector as the second argument
    );
    let review_exists = use_state(|| "block");

    let hide_fn = {
        let review_exists = review_exists.clone();
        let value = if *review_exists == "block" {
            "hidden"
        } else {
            "block"
        };
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            review_exists.set(value);
        })
    };  

    let onsubmit = Callback::from(move |user_review: UserReview| {
        web_sys::console::log_1(&format!("UserReview: {:?}", user_review.user_rating).into());
    });

    let restaurant_info = restaurant_info.as_ref().unwrap().clone();

    html! {
        <Layout>
            <div class="flex mt-3 border-3 items-center">
                <div class="flex-1">
                    <div class="flex flex-row">
                        <div class="mr-10 mt-4">
                            <BigCard description={restaurant_info.description.clone()} name={restaurant_info.name.clone()} />
                        </div>
                        <div class="w-2/3">
                            <h1 class="mb-2 text-4xl font-bold leading-tight text-primary">{ &restaurant_info.name }</h1>
                            <Rating is_loading={ false } num_star={ restaurant_info.num_star } />
                            <h3 class="mb-2 mt-3 text-3xl font-bold leading-tight text-primary">{"Write a review"}</h3>
                            <div class="w-3/4">
                                <WriteAReview onsubmit={ onsubmit.clone() } hide_fn={ hide_fn.clone() } initial_user_review={
                                    UserReview {
                                        user_rating: 5,
                                        user_review_title: String::from(""),
                                        user_review: String::from(""),
                                        user_name: String::from(""),
                                    }
                                } review_exists={ *review_exists } />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="w-3/4">
                // REVIEWS
                { render_reviews(user_reviews.as_ref()) }
            </div>
        </Layout>
    }
}   

async fn fetch_restuarant_data_async(
    name: String,
    user_reviews: UseStateHandle<Option<Vec<UserReview>>>,
    restaurant_info: UseStateHandle<Option<Rc<RestaurantInfo>>>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let url_reviews = format!("http://localhost:3000/restaurants/{}/reviews/", name);
        let url_info = format!("http://localhost:3000/restaurants/{}/", name);

        let reviews_response = match Request::get(&url_reviews).send().await {
            Ok(response) => response,
            Err(error) => {
                // Handle the error, e.g., log or display an error message
                web_sys::console::error_1(&format!("Error fetching reviews: {:?}", error).into());
                return;
            }
        };

        let info_response = match Request::get(&url_info).send().await {
            Ok(response) => response,
            Err(error) => {
                // Handle the error, e.g., log or display an error message
                web_sys::console::error_1(&format!("Error fetching restaurant info: {:?}", error).into());
                return;
            }
        };

        if !reviews_response.ok() || !info_response.ok() {
            // Handle non-OK responses, e.g., log or display an error message
            web_sys::console::error_1(&format!("Non-OK response received: reviews={:?}, info={:?}", reviews_response, info_response).into());
            return;
        }

        let reviews_json = match reviews_response.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(error) => {
                // Handle the error, e.g., log or display an error message
                web_sys::console::error_1(&format!("Error parsing reviews JSON: {:?}", error).into());
                return;
            }
        };

        let info_json = match info_response.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(error) => {
                // Handle the error, e.g., log or display an error message
                web_sys::console::error_1(&format!("Error parsing info JSON: {:?}", error).into());
                return;
            }
        };

        let fetched_reviews = match from_value::<ApiResponseForReviews>(reviews_json) {
            Ok(reviews) => reviews,
            Err(error) => {
                // Handle the error, e.g., log or display an error message
                web_sys::console::error_1(&format!("Error deserializing reviews: {:?}", error).into());
                return;
            }
        };

        let fetched_info = match from_value::<ApiResponseForInfo>(info_json) {
            Ok(info) => info,
            Err(error) => {
                // Handle the error, e.g., log or display an error message
                web_sys::console::error_1(&format!("Error deserializing info: {:?}", error).into());
                return;
            }
        };

        user_reviews.set(Some(fetched_reviews.data));
        restaurant_info.set(Some(Rc::new(fetched_info.data[0].clone())));
    });
}

fn render_reviews(user_reviews: Option<&Vec<UserReview>>) -> Html {
    match user_reviews {
        Some(reviews) => {
            html! {
                <>
                    <h3 class="mb-2 mt-3 text-3xl font-bold leading-tight text-primary">{"Other's Reviews"}</h3>
                    {
                        reviews.iter().map(|user| {
                            html! {
                                <>
                                    <Review
                                        user_rating={ user.user_rating }
                                        user_review_title={ user.user_review_title.clone() }
                                        user_review={ user.user_review.clone() }
                                        user_name={ user.user_name.clone() }
                                    />
                                    <hr class="my-5" />
                                    <br />
                                </>
                            }
                        }).collect::<Html>()
                    }
                </>
            }
        }
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
        }
    }
}
