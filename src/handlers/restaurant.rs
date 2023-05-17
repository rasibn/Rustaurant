use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};


use axum_login::axum_sessions::async_session::serde_json::value;
use bson::{Array, RawArray};
use futures::stream::StreamExt;

use mongodb::{
    bson::{Bson, doc, Document, oid::ObjectId, to_bson},
    Client,
    Collection,
    options::{FindOneOptions, DeleteOptions, UpdateOptions, FindOptions},
};
use serde_with::rust::unwrap_or_skip;


use crate::structs::restaurant::{Response, Restaurant, RestaurantDB, self};


pub async fn create_restaurant(State(client): State<Client>,Json(mut rest): Json<Restaurant>) -> impl IntoResponse {

    let rest_coll: Collection<RestaurantDB> = client
    .database("app_database")
    .collection::<RestaurantDB>("restaurant");
    
    let filter = doc! {
        "name": rest.name.clone(),
    };

    let payload = RestaurantDB {
        name: rest.name.clone(),
        description: rest.description.clone(),
        num_star:vec![Bson::Int32(0); 5],
    };

    let options = FindOneOptions::default();
    let cursor  = rest_coll.find_one(filter.clone(), options).await;
    //let cursor = users_coll.find_one(doc!{"email":payload.email.clone(),"username":payload.username.clone()}, options).await;

    match cursor {
        Ok(value) => {
            match value {
                Some(restaurant) => return {
                    (StatusCode::FOUND, Json(Response {
                        success: false,
                        error_message: Some("Restaurant already exists".to_string()),
                        data: None
                    }))
                },
                None => {
                    let result = rest_coll.insert_one(payload, None).await;
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
                                error_message: Some(format!("Couldn't create restaurant due to {:#?}", err)),
                                data: None
                            }))
                        }   
                    }
                }
            }
        },
        Err(err) => return {
            (StatusCode::NOT_FOUND, Json(Response {
                success: false,
                error_message: Some(format!("Couldn't find any restaurant due to {:#?}", err)),
                data: None
            }))
        }
    }
}

// pub async fn restaurant_from_substring(State(client): State<Client>,sub_name: String) -> impl IntoResponse {
//     let restaurant_name = name.0;
//     fetch_restaurant(client, doc! {
//         "name": { "$regex": &restaurant_name, "$options": "i" }
//     }).await
// }

pub async fn restaurant_from_name(State(client): State<Client>, name: Path<String>) -> impl IntoResponse {
    let restaurant_name = name.0;
    fetch_restaurant(client, doc! {
        "name": &restaurant_name
    }).await
}

async fn fetch_restaurant(client: Client, filter: Document) -> (StatusCode, Json<Response>) {

    let rest_coll: Collection<RestaurantDB> = client
    .database("app_database")
    .collection::<RestaurantDB>("restaurant");

    let options = FindOneOptions::default();

    let restaurant = rest_coll.find_one(filter.clone(), options).await;
    match restaurant {
        Ok(value) => {
            match value {
                Some(restaurant) => {
                    (StatusCode::FOUND, Json(Response {
                        success: true,
                        data: Some(vec![restaurant]),
                        error_message: None
                    }))
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
                        error_message: Some(format!("No restaurant exists for given filter: {}", message)),
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

pub async fn fetch_all_restaurant(State(client): State<Client>) -> impl IntoResponse {
    
    let rest_coll: Collection<RestaurantDB> = client
    .database("app_database")
    .collection::<RestaurantDB>("restaurant");

    let options = FindOptions::default();

    let mut restaurants_cursor = rest_coll
        .find(None, options).await.expect("could not load restaurants info.");

    let mut restaurants: Vec<RestaurantDB> = Vec::new();

    while let Some(doc) = restaurants_cursor.next().await {
        restaurants.push(doc.expect("could not load restaurant info."));
    }

    let response = Response {
        success: true,
        data: Some(restaurants),
        error_message: None
    };
    (StatusCode::OK, Json(response))


}