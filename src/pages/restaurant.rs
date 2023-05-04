use crate::components::card::Card;
use crate::components::layout::Layout;
use crate::components::rating::Rating;
use crate::components::review::Review;
use crate::components::review_modal::ReviewModal;
use crate::components::review_modal::UserReviews;
use yew::prelude::*;

#[function_component(Restaurant)]
pub fn restaurant() -> Html {
    // TODO: get restaurant name, address, description, and num_star from backend
    let restaurant_name = String::from("Restaurant A");
    let description = String::from("Restaurant A is a restaurant");
    let image_path = String::from("/images/dominos.jpg");
    let num_star = [50, 30, 13, 33, 52];

    // TODO: get a user's rating, review, user_image, user_join_date from backend
    let mut users: Vec<UserReviews> = Vec::new();

    users.push(UserReviews {
        user_rating: 5,
        user_review_title: String::from("This is a review title"),
        user_review: String::from("This is a review"),
        user_name: String::from("User A"),
        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
        user_join_date: String::from("2021-01-01"),
    });

    users.push(UserReviews {
        user_rating: 4,
        user_review_title: String::from("This is a review title"),
        user_review: String::from("This is a review"),
        user_name: String::from("User B"),
        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
        user_join_date: String::from("2021-01-01"),
    });

    users.push(UserReviews {
        user_rating: 3,
        user_review_title: String::from("This is a review title"),
        user_review: String::from("This is a review"),
        user_name: String::from("User C"),
        user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
        user_join_date: String::from("2021-01-01"),
    });
    // use_state in yew
    let show_modal = use_state(|| "block");

    // Make a onclick event to toggle the modal
    // let onsubmit = {
    //     let show_modal = show_modal.clone();
    //     let value = if *show_modal == "block" {
    //         "hidden"
    //     } else {
    //         "block"
    //     };
    //     Callback::from(move |e: MouseEvent| {
    //      e.prevent_default();
    //      show_modal.set(value)})
    // };

    let onsubmit = Callback::from(move |user_review: UserReviews| {
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
                            <ReviewModal {onsubmit} show_modal = {*show_modal}/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            //<button {onclick} class="block text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
            // {if *show_modal == "block" {"Submit"} else {"Leave a Review"}}
            // </button>
            <div class="w-3/4">
            <h3 class="mb-2 mt-3 text-3xl font-bold leading-tight text-primary">{"Other's Reviews"}</h3>
            {
                users.iter().map(|user| {
                    html! {
                        <>
                        <Review user_rating = {user.user_rating}
                                user_review_title = {user.user_review_title.clone()}
                                user_review = {user.user_review.clone()}
                                user_name = {user.user_name.clone()}
                                user_image = {user.user_image.clone()}
                                user_join_date = {user.user_join_date.clone()}/>
                        <br/>
                        </>
                    }
                }).collect::<Html>()
            }
            </div>
        </Layout>
    }
}
