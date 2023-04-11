use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};

use axum_login::axum_sessions::async_session::serde_json::value;
use futures::stream::StreamExt;

use mongodb::{
    bson::{Bson, doc, Document, oid::ObjectId,},
    Client,
    Collection,
    options::{FindOptions, FindOneOptions},
};


use crate::structs::user::{Response, User};

extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};
// use axum::{
//     http::StatusCode,
//     Json
// };

pub async fn create_user(State(client): State<Client>,Json(mut payload): Json<User>) -> impl IntoResponse {
    
    let encypted = hash(payload.password, DEFAULT_COST);
    payload.password = encypted.unwrap();

    let users_coll: Collection<User> = client
    .database("app_database")
    .collection::<User>("users");
    
    let options = FindOneOptions::default();
    let cursor = users_coll.find_one(doc!{"email":payload.email.clone()}, options).await;

    match cursor {
        Ok(value) => {
            match value {
                Some(user) => return {
                    (StatusCode::FOUND, Json(Response {
                        success: false,
                        error_message: Some("User already exists".to_string()),
                        data: None
                    }))
                },
                None => {
                    let result = users_coll.insert_one(payload.clone(), None).await;
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
                                error_message: Some(format!("Couldn't create user due to {:#?}", err)),
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
                error_message: Some(format!("Couldn't find any user due to {:#?}", err)),
                data: None
            }))
        }
    }









    // match cursor{
    //     Ok(_)=>{return (StatusCode::BAD_REQUEST, Json(Response {
    //         success: false,
    //         error_message: Some("User already exists".to_string()),
    //         data: None
    //     }))},
    //     Err(_)=>{
    //         let result = users_coll.insert_one(payload.clone(), None).await;
    //         match result {
    //             Ok(_) => {
    //                 (StatusCode::CREATED, Json(Response {
    //                     success: true,
    //                     error_message: None,
    //                     data: None
    //                 }))
    //             },
    //             Err(err) => {
    //                 (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
    //                     success: false,
    //                     error_message: Some(format!("Couldn't create user due to {:#?}", err)),
    //                     data: None
    //                 }))
    //             }   
    //         }
    //     }
    // }

    // let result = users_coll.insert_one(payload.clone(), None).await;
    // match result {
    //     Ok(_) => {
    //         (StatusCode::CREATED, Json(Response {
    //             success: true,
    //             error_message: None,
    //             data: None
    //         }))
    //     },
    //     Err(err) => {
    //         (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
    //             success: false,
    //             error_message: Some(format!("Couldn't create user due to {:#?}", err)),
    //             data: None
    //         }))
    //     }
    // }
}
// pub async fn create_user(
//     // this argument tells axum to parse the request body as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {     // insert your application logic here

//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }
