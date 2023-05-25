use crate::components::layout::Layout;
use serde_json::{Value, from_value, to_value};
use yew::prelude::*;
use crate::Route;
use yew_router::prelude::Link;
use gloo_net::http::Request;
use web_sys::window;
use crate::components::write_a_review::UserReview;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub route: String,
    pub review: UserReview,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct UserReviewForPost {
  pub user_rating: i32,
  pub user_review_title: String,
  pub restaurant_name: String,
  pub user_review: String,
  pub user_name: String,
}

#[function_component(Submitting)]
pub fn submitting(props: &Props) -> Html {
    let window = window().expect("Failed to retrieve window object");
    let route = vec!["restaurant", "KFC"]; // Replace with your desired route
    let review_cloned: UserReview = props.review.clone();

    use_effect_with_deps(
      move |_| {
          let restaurant_name = "KFC".to_string(); //TODO: remove hardcoded restaurant name

          let post_url = format!("http://localhost:3000/restaurants/{}/reviews/create/", restaurant_name);
          
          let review_to_post = UserReviewForPost {
              user_rating: review_cloned.user_rating,
              user_review_title: review_cloned.user_review_title,
              restaurant_name: restaurant_name,
              user_review: review_cloned.user_review,
              user_name: review_cloned.user_name,
          };

          wasm_bindgen_futures::spawn_local(async move {
              // Serialize the review as JSON
              let review_json = to_value(&review_to_post).unwrap();

              // Make a post request to send the review to the backend
              let request = Request::post(&post_url)
                  .header("Content-Type", "application/json")
                  .body(review_json.to_string());

              web_sys::console::log_1(&format!("Sending request: {:?}", request).into());

              // Handle the response/error of the post request
              match request.send().await {
                  Ok(response) => {
                      web_sys::console::log_1(&format!("Response: {:?}", response).into());
                      
                      let redirect_url = format!("/{}/{}", route[0], route[1]);
                      web_sys::console::log_1(&format!("Redirecting to: {:?}", redirect_url).into());
                       window.location().set_href(&redirect_url)
                             .expect("Failed to redirect");
                  }
                  Err(err) => {
                      // Handle the error when sending the request
                      web_sys::console::error_1(&format!("Failed to send request: {:?}", err).into());
                  }
              }
          });

          || {}
      },
      (),
  );

    html! {
        <Layout>
            <div class="flex flex-col items-center justify-center min-h-screen">
                <div class="max-w-3xl mx-auto text-white text-center">
                    <div class="flex items-center justify-center mb-8">
                        <div class="animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-white mr-2"></div>
                        <h1 class="text-5xl font-bold">{"Submitting..."}</h1>
                    </div>
                    <p class="text-gray-300 mb-8">{"You will be redirected to the review page shortly."}</p>
                    <Link<Route> to={Route::Home} classes="bg-blue-500 text-white font-semibold px-4 py-2 rounded hover:bg-blue-600">{"Go Back Home"}</Link<Route>>
                </div>
            </div>
        </Layout>
    }
}
