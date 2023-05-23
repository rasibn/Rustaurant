use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_rating: i32,
    pub user_review_title: String,
    pub user_review: String,
    pub user_name: String,
}

#[function_component(Review)]
pub fn reviews(props: &Props) -> Html {
    html! { 
        <article>
            <div class="flex items-center mb-4 space-x-4">
                <img class="w-10 h-10 rounded-full" src="/images/img_avatar.png" alt=""/>
                <div class="space-y-1 font-medium dark:text-white">
                    <p>{"Jese Leos"}<time datetime="2014-08-16 19:00" class="block text-sm text-gray-500 dark:text-gray-400">{"Joined today"}</time></p>
                </div>
            </div>
            <div class="flex items-center mb-1">

                {(0..5).into_iter().map(|i| {
                    let mut classes = "w-5 h-5 text-yellow-400";
                    if i+1 > props.user_rating {
                        classes = "w-5 h-5 text-gray-400";  
                    }
                    html! { 
                        <svg aria-hidden="true" class={classes} fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><title>{"First star"}</title><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"></path></svg>
                    }
                }).collect::<Html>()}

                <h3 class="ml-2 text-sm font-semibold text-gray-900 dark:text-white">{props.user_review_title.clone()}</h3>
            </div>
            <footer class="mb-5 text-sm text-gray-500 dark:text-gray-400"><p>{"Reviewed in the United Kingdom on "}<time datetime="2017-03-03 19:00">{"March 3, 2017"}</time></p></footer>

            <p class="mb-2 text-gray-500 dark:text-gray-400">{props.user_review.clone()}</p>

            <a href="#" class="block mb-5 text-sm font-medium text-blue-600 hover:underline dark:text-blue-500">{"Read more"}</a>
            <aside>
                <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">{"19 people found this helpful"}</p>
                <div class="flex items-center mt-3 space-x-3 divide-x divide-gray-200 dark:divide-gray-600">
                    <a href="#" class="text-gray-900 bg-white border border-gray-300 focus:outline-none hover:bg-gray-100 focus:ring-4 focus:ring-gray-200 font-medium rounded-lg text-xs px-2 py-1.5 dark:bg-gray-800 dark:text-white dark:border-gray-600 dark:hover:bg-gray-700 dark:hover:border-gray-600 dark:focus:ring-gray-700">{"Helpful"}</a>
                    <a href="#" class="pl-4 text-sm font-medium text-blue-600 hover:underline dark:text-blue-500">{"Report abuse"}</a>
                </div>
            </aside>
        </article>
    }
}
