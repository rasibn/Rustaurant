use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let routes = vec![
        ("Home", Route::Home),
        ("About", Route::About),
        ("Restaurant", Route::Restaurant {name: "KFC".to_string()}),
        // TODO: Change this later
        ("Random", Route::NotFound),
    ];
    html! {

    <nav class="bg-white dark:bg-gray-900 fixed w-full z-20 top-0 left-0 border-b border-gray-200 dark:border-gray-600">
    <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
    <a class="flex items-center">
        <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"Bitesy"}</span>
    </a>
    <div class="flex md:order-2 space-x-4">
        <Link<Route> to={Route::Login} classes="text-white border border-blue-600 hover:bg-blue-600  font-medium rounded-lg text-sm px-4 py-2 text-center mr-3 md:mr-0">{"LogIn"}</Link<Route>>
        <Link<Route> to={Route::CreateAccount} classes="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 text-center mr-3 md:mr-0 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{"Get started"}</Link<Route>>

        <button data-collapse-toggle="navbar-sticky" type="button" class="inline-flex items-center p-2 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-sticky" aria-expanded="false">
          <span class="sr-only">{"Open main menu"}</span>
          <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path></svg>
        </button>
    </div>
    <div class="items-center justify-between hidden w-full md:flex md:w-auto md:order-1" id="navbar-sticky">
      <ul class="flex flex-col p-4 md:p-0 mt-4 font-medium border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">

      {routes.into_iter().map(|route| html! {
        <li>
      <Link<Route> to={route.1} classes="block py-2 pl-3 pr-4 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 md:dark:hover:text-blue-500 dark:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700">{route.0}</Link<Route>>
    </li>
      }).collect::<Html>()}

      </ul>
    </div>
    </div>
    </nav>

      }
}
