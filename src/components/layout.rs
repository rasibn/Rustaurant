use yew::{function_component, html, Html, Properties, Children};

mod navbar;
mod footer;

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
        <div class="flex flex-col h-screen">
        <navbar::NavBar />
            <main class="mx-4 grow mt-20">
                { for props.children.iter()}
            </main>
        <footer::Footer />
        </div>
    }
}
 
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}
