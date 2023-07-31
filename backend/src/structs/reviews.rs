use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Serialize, Deserialize,)]
pub struct Review {
    pub restaurant_name: String,
    pub user_rating: i32,
    pub user_review_title: String,
    pub user_review: String,
    pub user_name: String,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub data: Option<Vec<Review>>,
    pub error_message: Option<String>
}
