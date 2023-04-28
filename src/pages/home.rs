use std::any;

use crate::components::home::{card::Card, card::Props as CardProps, search_input::SearchInput};
use crate::components::layout::Layout;
use yew::{function_component, html, use_effect, use_state_eq, Callback, Html, UseStateHandle};

#[function_component(Home)]
pub fn home() -> Html {
    let resuturants: UseStateHandle<Vec<CardProps>> = use_state_eq(|| Vec::new());

    // change to use_effect_with
    use_effect(move || {
        // TODO: call api to get data
        resuturants.set(vec![
            CardProps {
                name: "Dominos".to_string(),
                image: "/images/dominos.jpg".to_string(),
                description: "Dominos is a pizza restaurant".to_string(),
            },
            CardProps {
                name: "Dominos".to_string(),
                image: "/images/dominos.jpg".to_string(),
                description: "Dominos is a pizza restaurant".to_string(),
            },
            CardProps {
                name: "Dominos".to_string(),
                image: "/images/dominos.jpg".to_string(),
                description: "Dominos is a pizza restaurant".to_string(),
            },
        ]);
    });

    let onsubmit = Callback::from(move |search: String| {
        web_sys::console::log_1(&format!("Search ainput: {:?}", search).into());
    });

    html! {
        <Layout>
            <div>
                <div>
                    <SearchInput {onsubmit} />
                </div>
                <div class="grid grid-cols-4 gap-6 mx-16">
                // { resuturants.get(0).iter().map(|resturant| html!{<Card
                //         name="efwe"
                //         image="/images/dominos.jpg"
                //         description="Dominos is a pizza restaurant"
                //     />}) }
                    {for(0..10).map(|_| html!{<Card
                        name="Dominos"
                        image="/images/dominos.jpg"
                        description="Dominos is a pizza restaurant"
                    />})}
                </div>
            </div>
        </Layout>
    }
}
