use crate::components::card::Card;
use crate::components::layout::Layout;
use crate::components::rating::Rating;
use crate::components::review::Review;
use crate::components::write_a_review::ReviewModal;
use gloo_net::{http::Request, Error};
use serde::Deserialize;
use crate::components::write_a_review::UserReview;
use yew::prelude::*;

use yew::{
    function_component, html, use_effect_with_deps, use_state_eq, Callback, Html, Properties,
    UseStateHandle,
};

#[derive(Properties, Deserialize, PartialEq, Clone)]
pub struct UserReviews {
    pub user_reviews: Vec<UserReview>,
}
#[function_component(Restaurant)]
pub fn restaurant() -> Html {
    let user_reviews: UseStateHandle<Option<UserReviews>> = use_state_eq(|| None);

    {
        let user_reviews = user_reviews.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users = Request::get("https://dummyjson.com/users").send().await;
                    match fetched_users {
                        Ok(response) => {
                            web_sys::console::log_1(&format!("Response: {:?}", response).into());
                            user_reviews.set(Some(UserReviews {
                                user_reviews: (vec![
                                    UserReview {
                                        user_rating: 5,
                                        user_review_title: String::from("This is a review title"),
                                        user_review: String::from("This is a review"),
                                        user_name: String::from("User A"),
                                        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
                                        user_join_date: String::from("2021-01-01"),
                                    },
                                    UserReview {
                                        user_rating: 4,
                                        user_review_title: String::from("This is a review title"),
                                        user_review: String::from("This is a review"),
                                        user_name: String::from("User B"),
                                        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
                                        user_join_date: String::from("2021-01-01"),
                                    },
                                    UserReview {
                                        user_rating: 3,
                                        user_review_title: String::from("This is a review title"),
                                        user_review: String::from("This is a review"),
                                        user_name: String::from("User C"),
                                        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
                                        user_join_date: String::from("2021-01-01"),
                                    },
                                    UserReview {
                                        user_rating: 2,
                                        user_review_title: String::from("This is a review title"),
                                        user_review: String::from("This is a review"),
                                        user_name: String::from("User D"),
                                        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
                                        user_join_date: String::from("2021-01-01"),
                                    },
                                    UserReview {
                                        user_rating: 1,
                                        user_review_title: String::from("This is a review title"),
                                        user_review: String::from("This is a review"),
                                        user_name: String::from("User E"),
                                        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
                                        user_join_date: String::from("2021-01-01"),
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

    // TODO: get restaurant name, address, description, and num_star from backend
    let restaurant_name = String::from("Restaurant A");
    let description = String::from("Restaurant A is a restaurant");
    let image_path = String::from("/images/dominos.jpg");
    let num_star = [50, 30, 13, 33, 52];
    
    // use_state in yew
    let review_exists = use_state(|| "block");

    // Make a onclick event to toggle the modal

    let hide_fn = {
        let show_modal = review_exists.clone();
        let value = if *show_modal == "block" {
            "hidden"
        } else {
            "block"
        };
        Callback::from(move |e: MouseEvent| {
         e.prevent_default();
         show_modal.set(value)})
    };


    let onsubmit = Callback::from(move |user_review: UserReview| {
        web_sys::console::log_1(&format!("UserReview: {:?}", user_review.user_rating).into());
    });


    html! {
        <Layout>
            <div class="flex mt-3 border-3 items-center">
                <div class="flex-1">
                    <div class="flex flex-row">
                        <div class="mr-10">
                            <Card {description} name={restaurant_name.clone()} image={image_path} />
                        </div>
                        <div class="w-2/3">
                            <h1 class="mb-2 text-4xl font-bold leading-tight text-primary">{&restaurant_name}</h1>
                            <Rating is_loading={false} {num_star} />
                            <h3 class="mb-2 mt-3 text-3xl font-bold leading-tight text-primary">{"Write a review"}</h3>
                            <div class="w-3/4">
                            <ReviewModal {onsubmit} {hide_fn} initial_user_review={ //TODO: INITIAL REVIEW
                                UserReview {
                                user_rating: 5,
                                user_review_title: String::from("Origonal Review"),
                                user_review: String::from("MY INITIAL REVIEW"),
                                user_name: String::from("User ME"),
                                user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
                                user_join_date: String::from("2021-01-01"),
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
                    Some(resuturants) => resuturants
                        .user_reviews
                        .iter()
                        .map(|user| {
                            html! {
                                <>
                                <Review user_rating = {user.user_rating}
                                        user_review_title = {user.user_review_title.clone()}
                                        user_review = {user.user_review.clone()}
                                        user_name = {user.user_name.clone()}
                                        user_image = {user.user_image.clone()}
                                        user_join_date = {user.user_join_date.clone()}/>
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
