use crate::components::layout::Layout;
use crate::components::review::Review;
use crate::components::rating::Rating;

use yew::prelude::*;
// TODO: fix is_loading
#[function_component(Restaurant)]
pub fn restaurant() -> Html {

    html! { 
        <Layout>
            <h1 class="mb-2 mt-0 text-5xl font-medium leading-tight text-primary">{ "Restuarant A" }</h1>  
            <p class={classes!("")}> {" "}</p> 
            <br/> 
            <Rating is_loading = {false}
                    num_star= {[50, 30, 13, 33, 52]}
                    restaurant_name= {"Restaurant A".to_string()}
                    restaurant_description= {"Restaurant A is a restaurant".to_string()}
                    restaurant_address= {"123 Main St".to_string()}/>
            <br/>
            <h3 class="mb-2 mt-0 text-3xl font-medium leading-tight text-primary">{"Reviews"}</h3>
            <Review/>
        </Layout>
    }
}
