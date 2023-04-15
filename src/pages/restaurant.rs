use crate::components::layout::Layout;
use crate::components::review::Review;
use crate::components::rating::Rating;

use yew::prelude::*;

#[function_component(Restaurant)]
pub fn restaurant() -> Html {
    html! { 
        <Layout>
            <h1>{ "Restuarant A:" }</h1>  
            <p class={classes!("")}> {""}</p>  
            <Rating/>
            <p class={classes!("")}> {""}</p>  
            <Review/>
        </Layout>
    }
}
