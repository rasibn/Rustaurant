use crate::components::card::Card;
use crate::components::layout::Layout;
use crate::components::rating::Rating;
use crate::components::review::Review;
use crate::components::write_a_review::WriteAReview;
use gloo_net::{http::Request};
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
    let user_reviews: UseStateHandle<Option<Vec<UserReview>>> = use_state_eq(|| None);
    let restaurant_info: UseStateHandle<Option<Rc<RestaurantInfo>>> = use_state_eq(|| 
        Some(Rc::new(RestaurantInfo {
            name: String::from("Loading..."),
            description: String::from("Loading..."),
            num_star: [0; 5],
        }))
    );

    {
        let user_reviews = user_reviews.clone();
        let restaurant_info = restaurant_info.clone();
        let name = props.name.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let url = &format!("http://localhost:3000/restaurants/{}/reviews/", name)[..];
                    web_sys::console::log_1(&format!("URL: {:?}", url).into());
                    let reviews_response = Request::get(&format!("http://localhost:3000/restaurants/{}/reviews/", name)[..])
                        .send()
                        .await
                        .unwrap()
                        .json::<serde_json::Value>()
                        .await
                        .unwrap();
        
                    let info_response = Request::get(&format!("http://localhost:3000/restaurants/{}/", name)[..])
                        .send()
                        .await
                        .unwrap()
                        .json::<serde_json::Value>()
                        .await
                        .unwrap();
        
                    let fetched_reviews = from_value::<ApiResponseForReviews>(reviews_response).unwrap();
                    let fetched_info = from_value::<ApiResponseForInfo>(info_response).unwrap();
                    

                    web_sys::console::log_1(&format!("Fetched reviews: {:?}", fetched_reviews).into());
                    web_sys::console::log_1(&format!("Fetched info: {:?}", fetched_info).into());

                    user_reviews.set(Some(fetched_reviews.data));
                    restaurant_info.set(Some(Rc::new(fetched_info.data[0].clone())));
                });
            },
            (),
        );
    } 

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
         review_exists.set(value)})
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
                        <div class="mr-10">
                            <Card description={restaurant_info.description.clone()} name={restaurant_info.name.clone()}/>
                        </div>
                        <div class="w-2/3">
                            <h1 class="mb-2 text-4xl font-bold leading-tight text-primary">{&restaurant_info.name}</h1>
                            <Rating is_loading={false} num_star={restaurant_info.num_star} />
                            <h3 class="mb-2 mt-3 text-3xl font-bold leading-tight text-primary">{"Write a review"}</h3>
                            <div class="w-3/4">
                            <WriteAReview {onsubmit} {hide_fn} initial_user_review={ //TODO: INITIAL REVIEW
                                UserReview {
                                user_rating: 5,
                                user_review_title: String::from("Initial Review"),
                                user_review: String::from("My own initial review goes here"),
                                user_name: String::from("User ME"),
                            }}
                            review_exists = {*review_exists}/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="w-3/4">
            // REVIEWS 
            <h3 class="mb-2 mt-3 text-3xl font-bold leading-tight text-primary">{"Other's Reviews"}</h3>
            {
                match user_reviews.as_ref() {
                    Some(restaurants) => restaurants
                        .iter()
                        .map(|user| {
                            html! {
                                <>
                                <Review user_rating = {user.user_rating}
                                        user_review_title = {user.user_review_title.clone()}
                                        user_review = {user.user_review.clone()}
                                        user_name = {user.user_name.clone()}/>
                                <hr class="my-5" />
                                <br/>
                                </>  
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
                }
            }
            </div>
        </Layout>
    }
}
