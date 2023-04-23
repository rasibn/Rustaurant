use yew::{function_component, html, Html, Properties, Callback, MouseEvent, InputEvent, use_state};
use web_sys::{Event, HtmlInputElement};
use std::ops::Deref;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub show_modal: String,
    pub onsubmit: Callback<UserReviews>,
}
#[derive(Debug)]
pub struct UserReviews {
    pub user_rating: i32,
    pub user_review_title: String,
    pub user_review: String,
    pub user_name: String,
    pub user_image: String,
    pub user_join_date: String,
}

#[function_component(ReviewModal)]
pub fn review_modal(props: &Props) -> Html {
    let my_text_handle_title = use_state(|| "".to_string());
    let my_text_handle_review = use_state(|| "".to_string());
    let my_text_handle_rating = use_state(|| "1".to_string()); // Set the default value to "1" for rating

    let cloned_title = my_text_handle_title.deref().clone();
    let cloned_review = my_text_handle_review.deref().clone();
    let cloned_rating = my_text_handle_rating.deref().clone();

    let handle_input_title = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap();
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value = input_elem.value();
        web_sys::console::log_1(&format!("Title: {:?}", value).into());
        my_text_handle_title.set(value); // update as user types 
    });

    let handle_input_review = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap();
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value = input_elem.value();
        web_sys::console::log_1(&format!("Review: {:?}", value).into());
        my_text_handle_review.set(value); // update as user types
    });

    let handle_input_rating = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap();
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value = input_elem.value();
        web_sys::console::log_1(&format!("Rating: {:?}", value).into());
        my_text_handle_rating.set(value); // update as user types
    });

    let user_review = UserReviews {
        user_review_title: cloned_title.clone(),
        user_review: cloned_review.clone(),
        user_rating: cloned_rating.parse::<i32>().unwrap_or(-1), // rating is greater than 5 just set it to -1
        user_name: "John Doe".to_string(),
        user_image: "https://i.imgur.com/8Km9tLL.jpg".to_string(),
        user_join_date: "2021-01-01".to_string(),
    };

    let onsubmit = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        web_sys::console::log_1(&format!("User Review: {:?}", user_review).into());
        // props.onsubmit.emit(&user_review);
    });

    html! {  //<!-- Main modal -->
    <form>
    <div class ={props.show_modal.clone()}>
    <div>
    <label for="small-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Review Title"}</label>
    <input type="text" placeholder={"Put your review title here"} value={cloned_title} oninput={handle_input_title} id="small-input" class="block w-full p-2 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 sm:text-xs focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
    </div>
    <div class="mb-6">
    <label for="large-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"My Review"}</label>
    <input type="text" placeholder={"The world is waiting for your review."} value={cloned_review} oninput={handle_input_review} id="large-input" class="block w-full p-4 text-gray-900 border border-gray-300 rounded-lg bg-gray-50 sm:text-md focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
    </div>
    <div class="mb-6">
    //<label for="default-input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Default input"}</label>
    //<input type="text" id="default-input" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"/>
    </div>
    <div class="mb-6">
    <label for="my-dropdown" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Select a rating:"}</label>
    <input id="my-dropdown" placeholder={"Rate this rustaurant 1 to 5"} value={cloned_rating.to_string()} oninput={handle_input_rating} class="z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700" name="my-dropdown"/>  
    </div>
    </div>
    <input onclick={onsubmit} type="submit" value={"Submit"} class="block text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"/>
    </form>

    }
}
