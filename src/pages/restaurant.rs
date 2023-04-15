use crate::components::layout::Layout;
use crate::components::review::Review;
use crate::components::rating::Rating;

use yew::prelude::*;
// TODO: Add props
#[function_component(Restaurant)]
pub fn restaurant() -> Html {
    html! { 
        <Layout>
            <h1 class="mb-2 mt-0 text-5xl font-medium leading-tight text-primary">{ "Restuarant A" }</h1>  
            <p class={classes!("")}> {" "}</p> 
            <br/> 
            <Rating/>
            <br/>
            <h3 class="mb-2 mt-0 text-3xl font-medium leading-tight text-primary">{"Reviews"}</h3>
            <Review/>
        </Layout>
    }
}
