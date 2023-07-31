use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="w-full mt-4 bg-white rounded-lg shadow dark:bg-gray-900">
            <div class="w-full max-w-screen-xl mx-auto p-4 md:py-8">
                <div class="sm:flex sm:items-center sm:justify-between">
                    <ul class="flex flex-wrap items-center mb-6 text-sm font-medium text-gray-500 sm:mb-0 dark:text-gray-400">
                        <li>
                            <Link<Route> to={Route::About} classes="mr-4 hover:underline md:mr-6 ">{"About"}</Link<Route>>
                        </li>
                        <li>
                            <span class="mr-4 hover:underline md:mr-6" title="View Privacy Policy">{"Privacy Policy"}</span>
                        </li>
                        <li>
                            <span class="mr-4 hover:underline md:mr-6 " title="View Licensing Information">{"Licensing"}</span>
                        </li>
                        <li>
                            <span class="hover:underline" title="Contact Us">{"Contact"}</span>
                        </li>
                    </ul>
                </div>
            </div>
        </footer>
    }
}
