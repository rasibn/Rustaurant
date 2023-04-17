use crate::components::layout::Layout;
use crate::components::review::Review;
use crate::components::rating::Rating;
use crate::components::description::Description;
use yew::prelude::*;
// TODO: fix is_loading

#[derive(Clone, PartialEq)]
struct UserReview {
    user_rating: i32,
    user_review_title: String,
    user_review: String,
    user_name: String,
    user_image: String,
    user_join_date: String,
}
#[function_component(Restaurant)]
pub fn restaurant() -> Html {
    // TODO: get restaurant name, address, description, and num_star from backend
    let restaurant_name = String::from("Restaurant A");
    let description = String::from("Restaurant A is a restaurant");
    let num_star = [50, 30, 13, 33, 52]; 

    // TODO: get a user's rating, review, user_image, user_join_date from backend
    let mut users: Vec<UserReview> = Vec::new();

    users.push(
        UserReview {
            user_rating: 5,
            user_review_title: String::from("This is a review title"),
            user_review: String::from("This is a review"),
            user_name: String::from("User A"),
            user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
            user_join_date: String::from("2021-01-01"),
        }
    );

    users.push(
        UserReview {
            user_rating: 4,
            user_review_title: String::from("This is a review title"),
            user_review: String::from("This is a review"),
            user_name: String::from("User B"),
            user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
            user_join_date: String::from("2021-01-01"),
        }
    );

    users.push(
        UserReview {
            user_rating: 3,
            user_review_title: String::from("This is a review title"),
            user_review: String::from("This is a review"),
            user_name: String::from("User C"),
            user_image: String::from("https://www.w3schools.com/howto/img_avatar.png"),
            user_join_date: String::from("2021-01-01"),
        }
    );


    html! { 
        <Layout>
            <h1 class="mb-2 mt-20 text-5xl font-medium leading-tight text-primary">{&restaurant_name}</h1>  
            //<p class={classes!("")}> {" "}</p> 
            <br/>
            <div class="flex">
                <div class="flex-none mr-3">
                    <Description {description}/>
                </div>
                <div class="flex-1 w-52">
                    <Rating is_loading = {false}
                    {num_star}/>
                </div>
            </div>
            <br/>
            <h3 class="mb-2 mt-0 text-3xl font-medium leading-tight text-primary">{"Reviews"}</h3>
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
        </Layout>
    }
}
