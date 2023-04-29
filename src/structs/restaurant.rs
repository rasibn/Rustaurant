use bson::Array;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Serialize, Deserialize,)]
pub struct Restaurant {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize,)]
pub struct RestaurantDB {
    pub name: String,
    pub description: String,
    pub num_star: Array,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub data: Option<Vec<RestaurantDB>>,
    pub error_message: Option<String>
}
