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


#[derive(Deserialize, Debug)]
pub struct ApiResponseForInfo {
    data: Vec<RestaurantInfo>,
    success: bool,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponseForReviews {
    data: Vec<UserReview>,
    success: bool,
}

#[derive(Deserialize, Debug)]
struct RestaurantInfo {
    name: String,
    description: String,
    num_star: [i32; 5],
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct UserReviews {
    pub user_reviews: Vec<UserReview>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(Restaurant)]
pub fn restaurant(props: &Props) -> Html {
    let user_reviews: UseStateHandle<Option<UserReviews>> = use_state_eq(|| None);

    {
        let user_reviews = user_reviews.clone();
        let name = props.name.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users = Request::get("https://dummyjson.com/users").send().await;
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

                    web_sys::console::log_1(&format!("Fetched resturants: {:?}", fetched_reviews).into());
                    web_sys::console::log_1(&format!("Fetched info: {:?}", fetched_info).into());

                    match fetched_users {
                        Ok(response) => {
                            //web_sys::console::log_1(&format!("Response: {:?}", response).into());
                            user_reviews.set(Some(UserReviews {
                                user_reviews: (vec![
                                    UserReview {
                                        user_rating: 5,
                                        user_review_title: String::from("This is a review title"),
                                        user_review: String::from("This is my third Invicta Pro Diver. They are just fantastic value for money. This one arrived yesterday and the first thing I did was set the time, popped on an identical strap from another Invicta and went in the shower with it to test the waterproofing.... No problems.
                                        
                                        It is obviously not the same build quality as those very expensive watches. But that is like comparing a Citroën to a Ferrari. This watch was well under £100! An absolute bargain.
                                        
                                        "),
                                        user_name: String::from("User A"),
                                    },
                                    UserReview {
                                        user_rating: 4,
                                        user_review_title: String::from("This is a review title"),
                                        user_review: String::from("
                                        "),                                        user_name: String::from("User B"),
                                    },
                                    UserReview {
                                        user_rating: 3,
                                        user_review_title: String::from("Thinking to buy another one!"),
                                        user_review: String::from("This is my third Invicta Pro Diver. They are just fantastic value for money. This one arrived yesterday and the first thing I did was set the time, popped on an identical strap from another Invicta and went in the shower with it to test the waterproofing.... No problems.
                                        
                                        It is obviously not the same build quality as those very expensive watches. But that is like comparing a Citroën to a Ferrari. This watch was well under £100! An absolute bargain.
                                        
                                        "),                                        user_name: String::from("User C"),
                                    },
                                    UserReview {
                                        user_rating: 2,
                                        user_review_title: String::from("Thinking to buy another one!"),
                                        user_review: String::from("This is my third Invicta Pro Diver. They are just fantastic value for money. This one arrived yesterday and the first thing I did was set the time, popped on an identical strap from another Invicta and went in the shower with it to test the waterproofing.... No problems.
                                        
                                        It is obviously not the same build quality as those very expensive watches. But that is like comparing a Citroën to a Ferrari. This watch was well under £100! An absolute bargain.
                                        
                                        "),                                        user_name: String::from("User D"),
                                    },
                                    UserReview {
                                        user_rating: 1,
                                        user_review_title: String::from("Thinking to buy another one!"),
                                        user_review: String::from("This is my third Invicta Pro Diver. They are just fantastic value for money. This one arrived yesterday and the first thing I did was set the time, popped on an identical strap from another Invicta and went in the shower with it to test the waterproofing.... No problems.
                                        
                                        It is obviously not the same build quality as those very expensive watches. But that is like comparing a Citroën to a Ferrari. This watch was well under £100! An absolute bargain.
                                        
                                        "),                                        user_name: String::from("User E"),
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
                    Some(resuturants) => resuturants
                        .user_reviews
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
