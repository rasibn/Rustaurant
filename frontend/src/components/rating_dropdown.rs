use yew::{function_component, html, Html, Properties};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub show_dropdown: String,
}

#[function_component(RatingDropdown)]
pub fn rating_dropdown(props: &Props) -> Html {
    html! {
        <div class={props.show_dropdown.clone()}>
            <div id="dropdown" class="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700">
                <ul class="py-2 text-sm text-gray-700 dark:text-gray-200">
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Dashboard"}</a>
                </li>
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Settings"}</a>
                </li>
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Earnings"}</a>
                </li>
                <li>
                    <a href="#" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">{"Sign out"}</a>
                </li>
                </ul>
            </div>
        </div>
    }
}
 





