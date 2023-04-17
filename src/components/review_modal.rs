use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub show_modal: String,
}

#[function_component(ReviewModal)]
pub fn review_modal(props: &Props) -> Html { //TODO: Dummy code fix later
    html! { 
        //<!-- Main modal -->
        <div class={props.show_modal.clone()}>{"HIFHAIHFDSAHOIFDSAHFDSOIHFDHOFDS"}</div> 
    }
}
