use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<String>,
}

#[function_component(SearchInput)]
pub fn search_input(props: &Props) -> Html {
    let my_text_handle_search = use_state(|| "".to_string());

    let cloned_search = my_text_handle_search.deref().clone();

    let handle_input_search = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap();
        let input_elem: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
        let value = input_elem.value();
        web_sys::console::log_1(&format!("Search: {:?}", value).into());
        my_text_handle_search.set(value); // update as user types
    });

    let search_value = cloned_search.clone();

    html! {
        <form class="flex mx-16 mb-5 justify-center">
            <label for="search" class="sr-only">{"Search"}</label>
            <div class="relative w-1/2">
                <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                    <svg aria-hidden="true" class="w-5 h-5 text-gray-500 dark:text-gray-400" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd"></path></svg>
                </div>
                <input type="text" id="search" value={cloned_search} oninput={handle_input_search} class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Search" />
            </div>
            <Link<Route> to={Route::Redirecting { route: format!("search+{}", search_value) }} >
                <button type="submit" class="p-2.5 ml-2 text-sm font-medium text-white bg-blue-700 rounded-lg border border-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>        <span class="sr-only">{ "Search" }</span>
                </button>
            </Link<Route>>
        </form>
    }
}
