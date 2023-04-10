use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// // the input to our `create_user` handler
// #[derive(Deserialize)]
// pub struct CreateUser {
//     pub username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// pub struct User {
//     pub id: u64,
//     pub username: String,
// }

#[derive(Clone, Debug, Serialize, Deserialize,)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub data: Option<Vec<User>>,
    pub error_message: Option<String>
}
