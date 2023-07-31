use crate::components::layout::Layout;
use serde_json::{Value, from_value, to_value};
use yew::prelude::*;
use crate::Route;
use yew_router::prelude::Link;
use gloo_net::http::Request;
use web_sys::window;
use crate::components::write_a_review::UserReview;
use crate::utils::hex_to_struct;
use serde::{Deserialize, Serialize};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub review_hex: String,
}

#[function_component(Submitting)]
pub fn submitting(props: &Props) -> Html {
    let window = window().expect("Failed to retrieve window object");
    let review_from_hex: UserReview = hex_to_struct(props.review_hex.as_str()).unwrap();
    let restaurant_name = review_from_hex.restaurant_name.clone();
    let restaurant_name_cloned = review_from_hex.restaurant_name.clone();

    web_sys::console::log_1(&format!("Review from hex: {:?}", review_from_hex).into());
    use_effect_with_deps(
      move |_| {
          let post_url = format!("http://localhost:3000/restaurants/{}/reviews/create/", restaurant_name);
          
          let review_to_post = UserReview {
              user_rating: review_from_hex.user_rating,
              user_review_title: review_from_hex.user_review_title,
              restaurant_name: restaurant_name,
              user_review: review_from_hex.user_review,
              user_name: review_from_hex.user_name,
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
                      
                      let redirect_url = format!("/restaurant/{}", restaurant_name_cloned);
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
