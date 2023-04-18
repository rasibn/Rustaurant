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
    options::{FindOneOptions, DeleteOptions, UpdateOptions},
};


use crate::structs::user::{Response, User, self};

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
    
    let filter = doc! {
        "$or": [
            {"username": payload.username.clone()},
            {"email": payload.email.clone()}
        ]
    };
    
    let options = FindOneOptions::default();
    let cursor = users_coll.find_one(filter, options).await;
    //let cursor = users_coll.find_one(doc!{"email":payload.email.clone(),"username":payload.username.clone()}, options).await;

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
}

pub async fn delete_user(State(client): State<Client>, email: Path<String>) -> impl IntoResponse {
    let users_coll: Collection<User> = client
    .database("app_database")
    .collection::<User>("users");

    let options = DeleteOptions::default();
    let result = users_coll.delete_one(doc!{"email":email.0}, options).await;

    match result {
        Ok(value) => {
            match value.deleted_count {
                0 => {
                    (StatusCode::NOT_FOUND, Json(Response {
                        success: false,
                        error_message: Some("No user found".to_string()),
                        data: None
                    }))
                },
                _ => {
                    (StatusCode::OK, Json(Response {
                        success: true,
                        error_message: None,
                        data: None
                    }))
                }
            }
        },
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Response {
                success: false,
                error_message: Some(format!("Couldn't delete user due to {:#?}", err)),
                data: None
            }))
        }
    }
        
}

pub async fn user_from_email(State(client): State<Client>, email: Path<String>) -> impl IntoResponse {
    let user_email = email.0;
    fetch_user(client, doc! {
        "email": &user_email
    }).await
}

pub async fn user_from_username(State(client): State<Client>, name: Path<String>) -> impl IntoResponse {
    let user_name = name.0;
    fetch_user(client, doc! {
        "username": &user_name
    }).await
}

async fn fetch_user(client: Client, filter: Document) -> (StatusCode, Json<Response>) {

    let users_coll: Collection<User> = client
    .database("app_database")
    .collection::<User>("users");

    let mut options = FindOneOptions::default();
    options.projection = Some(doc! {
        "username": 1,
        "email": 1,
        "password": 1
    });

    let user = users_coll.find_one(filter.clone(), options).await;
    match user {
        Ok(value) => {
            match value {
                Some(user) => {
                    (StatusCode::FOUND, Json(Response {
                        success: true,
                        data: Some(vec![user]),
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
                        error_message: Some(format!("No user exists for given filter: {}", message)),
                        data: None
                    }))
                }
            }
        },
        Err(err) => {
            (StatusCode::NOT_FOUND, Json(Response {
                success: false,
                error_message: Some(format!("Couldn't find any user due to {:#?}", err)),
                data: None
            }))
        }
    }
}

pub async fn update_user (State(client): State<Client>,Json(payload): Json<User>) -> impl IntoResponse {
    let users_coll: Collection<User> = client
    .database("app_database")
    .collection::<User>("users");

    let filter = doc! {
        "email": payload.email.clone()
    };

    let update = doc! { 
        "$set": 
        { "username": payload.username.clone() } 
    };

    let status = fetch_user(client, doc! {"username":payload.username.clone()}).await;

    match status {
        (StatusCode::NOT_FOUND, _) => {
            let options = UpdateOptions::default();
            let cursor = users_coll.update_one(filter, update, options).await;
            //let cursor = users_coll.find_one_and_update(filter, update,options).await;
            //let cursor = users_coll.find_one(doc!{"email":payload.email.clone(),"username":payload.username.clone()}, options).await;
        
            match cursor {
                Ok(value) => {
                    match value.modified_count {
                        0 => {
                            (StatusCode::NOT_FOUND, Json(Response {
                                success: false,
                                error_message: Some("No user updated or found".to_string()),
                                data: None
                            }))
                        },
                        _ => {
                            (StatusCode::OK, Json(Response {
                                success: true,
                                error_message: None,
                                data: None
                            }))
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
        
        },
        _ => return {
            (StatusCode::BAD_REQUEST, Json(Response {
                success: false,
                error_message: Some(format!("Username already exists")),
                data: None
            }))
        }


    }

    // let options = UpdateOptions::default();
    // let cursor = users_coll.update_one(filter, update, options).await;
    // //let cursor = users_coll.find_one_and_update(filter, update,options).await;
    // //let cursor = users_coll.find_one(doc!{"email":payload.email.clone(),"username":payload.username.clone()}, options).await;

    // match cursor {
    //     Ok(value) => {
    //         match value.modified_count {
    //             0 => {
    //                 (StatusCode::NOT_FOUND, Json(Response {
    //                     success: false,
    //                     error_message: Some("No user updated or found".to_string()),
    //                     data: None
    //                 }))
    //             },
    //             _ => {
    //                 (StatusCode::OK, Json(Response {
    //                     success: true,
    //                     error_message: None,
    //                     data: None
    //                 }))
    //             }
    //         }
    //     },
    //     Err(err) => return {
    //         (StatusCode::NOT_FOUND, Json(Response {
    //             success: false,
    //             error_message: Some(format!("Couldn't find any user due to {:#?}", err)),
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
