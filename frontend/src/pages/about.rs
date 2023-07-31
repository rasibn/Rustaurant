use yew::{function_component, html, Html};
use crate::components::{layout::Layout};

#[function_component(About)]
pub fn about() -> Html {
    html! { 
        <Layout>
            <div class="max-w-3xl mx-auto py-10">
                <h1 class="text-3xl text-white font-bold mb-5">{"About us"}</h1>
                <p class="mb-8 text-white">{"Welcome to Rustaurant, the most delicious and entertaining restaurant review app in the digital world! With Rust's blazingly fast speed, we'll satisfy your cravings for both great food and efficient software. Join our community of food enthusiasts as we embark on a gastronomic adventure together, powered by Rust's speed and reliability."}</p>
                <div class="bg-white shadow-md rounded-lg px-5 py-4">
                    <h2 class="text-lg font-medium mb-3">{"Meet the team"}</h2>
                    <ul class="list-disc list-inside">
                        <li>{"Rasib Nadeem - Master of Flavor"}</li>
                        <li>{"Shariq Anwar - Connoisseur of Cuisine"}</li>
                        <li>{"Taha Mirza - Wizard of Taste"}</li>
                    </ul>
                </div>
            </div>
        </Layout> 
    }
}
