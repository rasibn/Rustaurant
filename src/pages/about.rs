use yew::{function_component, html, Html};
use crate::components::{layout::Layout};

#[function_component(About)]
pub fn about() -> Html {
    html! { 
        <Layout>
            <div class="max-w-3xl mx-auto py-10">
                <h1 class="text-3xl text-white font-bold mb-5">{"About us"}</h1>
                <p class="mb-8 text-white">{"This is a restaurant review app. It is a work in progress."}</p>
                <div class="bg-white shadow-md rounded-lg px-5 py-4">
                    <h2 class="text-lg font-medium mb-3">{"Meet the team"}</h2>
                    <ul class="list-disc list-inside">
                        <li>{"Rasib Nadeem - CEO"}</li>
                        <li>{"Shariq Anwar - COO"}</li>
                        <li>{"Taha Mirza - CTO"}</li>
                    </ul>
                </div>
            </div>
        </Layout> 
    }
}
