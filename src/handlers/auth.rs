use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json
};

use futures::stream::StreamExt;

use mongodb::{
    bson::{Bson, doc, Document, oid::ObjectId},
    Client,
    Collection,
    options::{FindOptions, FindOneOptions},
};

use crate::structs::mflix::{Pagination, Response, SampleUser};


/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}


pub async fn login(State(client): State<Client>, user: Json<SampleUser>) -> impl IntoResponse {

    let users_coll: Collection<SampleUser> = client
        .database("sample_mflix")
        .collection::<SampleUser>("users");

    let mut options = FindOneOptions::default();

    options.projection = Some(doc! {
        "name": 1,
        "email": 1
    });

    let user = users_coll.find_one(doc! {
        "email": &user.email,
        "password": &user.password
    }, options).await;



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
                    (StatusCode::NOT_FOUND, Json(Response {
                        success: false,
                        error_message: Some("No user exists for given filter.".to_owned()),
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


// todo: add signup
pub async fn signup(State(client): State<Client>, user: Json<SampleUser>) -> impl IntoResponse {

    let users_coll: Collection<SampleUser> = client
        .database("sample_mflix")
        .collection::<SampleUser>("users");

    let mut options = FindOneOptions::default();

    options.projection = Some(doc! {
        "name": 1,
        "email": 1
    });

    let user = users_coll.find_one(doc! {
        "email": &user.email
    }, options).await;





}


