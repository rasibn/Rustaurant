use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
    pub num_star: [i32; 5],
}

#[function_component(Rating)]
pub fn rating(props: &Props) -> Html {
    //panic!("num_star: {:?}", props.num_star);

    let num_reviews = props.num_star.iter().fold(0, |acc, &num| acc + num);

    let avg_rating = props.num_star.iter().enumerate().fold(0.0, |acc, (i, &num)| {
        acc + (i as f32 + 1.0) * num as f32
    }) / num_reviews as f32;

    let avg_rating = (avg_rating*100.0).round()/100.0;
    let avg_rating_int = avg_rating as i32;
    
    let star_class = (0..5).into_iter().map(|i| {
        if avg_rating_int > i { "w-5 h-5 text-yellow-400" } else { "w-5 h-5 text-gray-300 dark:text-gray-500" }.to_string()
    }).collect::<Vec<String>>();

    html! {
    <div>
    <div class="flex items-center mb-3">

        {star_class.iter().map(|star| html! {
            <svg aria-hidden="true" class={star} fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><title>{"Star"}</title><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"></path></svg>
        }).collect::<Html>()}

        <p class="ml-2 text-sm font-medium text-gray-900 dark:text-white">{&avg_rating}{" out of 5"}</p>
    </div>
    <p class="text-sm font-medium text-gray-500 dark:text-gray-400">{&num_reviews}{" global ratings"}</p>
    {
       props.num_star.iter().enumerate().map(|tuple| {
        let (i, num_star) = tuple;
        let rating_percent = (*num_star as f32 / num_reviews as f32) * 100.0;
        let rating_percent_int = rating_percent.round() as i32;            
        html! {
                <div class="flex items-center mt-4">
                <span class="text-sm font-medium text-blue-600 dark:text-blue-500">{i+1}{" star"}</span>
                <div class="w-2/4 h-5 mx-4 bg-gray-200 rounded dark:bg-gray-700">
                    <div class="h-5 bg-yellow-400 rounded" style={format!("width: {}%", &rating_percent_int)}></div>
                </div>
                <span class="text-sm font-medium text-blue-600 dark:text-blue-500">{&rating_percent_int}{"%"}</span>
                </div>
            }
        }).collect::<Html>()

    }

    </div>}
}
