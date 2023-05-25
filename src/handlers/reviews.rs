use axum::{
    extract::{Path,State},
    http::StatusCode,
    response::IntoResponse,
    Json
};

use futures::stream::StreamExt;

use mongodb::{
    bson::{Bson, doc},
    Client,
    Collection,
    options::{FindOneOptions, DeleteOptions, UpdateOptions, FindOptions},
};
use crate::structs::reviews::{Review, Response, self};
use crate::structs::restaurant::{RestaurantDB};

pub async fn create_review(State(client): State<Client>,Json(mut rest): Json<Review>) -> impl IntoResponse {
    let review_coll: Collection<Review> = client
    .database("app_database")
    .collection::<Review>("reviews");

    let rating:usize = rest.user_rating as usize;

    if rating > 5 || rating < 1 {
        rest.user_rating = 5;
        // return (StatusCode::BAD_REQUEST, Json(Response {
        //     success: false,
        //     error_message: Some("Rating must be between 1 and 5".to_string()),
        //     data: None
        // }))
    }

    let filter = doc! {
        "name": rest.restaurant_name.clone(),
    }; 

    let rest_coll: Collection<RestaurantDB> = client
    .database("app_database")
    .collection::<RestaurantDB>("restaurant");

    let options = FindOneOptions::default();

    let restaurant = rest_coll.find_one(filter.clone(), options).await;

    match restaurant {
        Ok(value) => {
            match value {
                Some(mut restaurant) => {
                    let old_restaurant = doc! {
                        "name": restaurant.name.clone(),
                        "description": restaurant.description.clone(),
                        "num_star": restaurant.num_star.clone(),
                    };
                    let arr = &mut restaurant.num_star;
                    if let Some(reference) = arr.get_mut(rating-1){
                        *reference = Bson::Int32((*reference).as_i32().unwrap()+1);
                    };
                    let cursor = rest_coll.replace_one(old_restaurant, restaurant, None).await;
                    let result = review_coll.insert_one(&rest, None).await;
                    match cursor {
                        Ok(_) => {
                            match result {
                                Ok(_) => {
                                    (StatusCode::CREATED, Json(Response {
                                        success: true,
                                        error_message: None,
                                        data: None
                                    }))
                                },
                                Err(err) => {
                                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
                                        success: false,
                                        error_message: Some(format!("Couldn't post review due {:#?}", err)),
                                        data: None
                                    }))
                                }   
                            }
                        },
                        Err(_) => {
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
                                success: false,
                                error_message: Some(format!("Internal server error")),
                                data: None
                            }))
                        }   
                    }
                },
                None => {
                    let mut message: String = "".to_owned();
                    for (k, v) in filter {
                        let message_part = match v {
                            Bson::String(val) => format!("{}=={}, ", k, val),
                            _ => format!("{}=={}, ", k, v)
                        };
                        message.push_str(&message_part);
                    }
                    (StatusCode::NOT_FOUND, Json(Response {
                        success: false,
                        error_message: Some("Restaurant does not exists".to_string()),
                        data: None
                    }))
                }
            }
        },
        Err(err) => {
            (StatusCode::NOT_FOUND, Json(Response {
                success: false,
                error_message: Some(format!("Couldn't find any restaurants due to {:#?}", err)),
                data: None
            }))
        }
    }

}

pub async fn get_reviews_from_restaurant(State(client): State<Client>, Path(name): Path<String>) -> impl IntoResponse{
    
    let review_coll: Collection<Review> = client
    .database("app_database")
    .collection::<Review>("reviews");

    let filter = doc! {
        "restaurant_name": &name
    };

    
    let options = FindOptions::default();

    let reviews = review_coll.find(filter.clone(), options).await;
    let mut all_review: Vec<Review> = Vec::new();

    match reviews {
        Ok(mut cursor) => {
            while let Some(doc) = cursor.next().await {
                all_review.push(doc.expect("could not load reviews info."));
            }
            (StatusCode::OK, Json(Response {
                success: true,
                error_message: None,
                data: Some(all_review)
            }))
        },
        Err(err) => {
            (StatusCode::NOT_FOUND, Json(Response {
                success: false,
                error_message: Some(format!("Couldn't find any reviews due to {:#?}", err)),
                data: None
            }))
        }
    }

}
