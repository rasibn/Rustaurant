use axum::{
    routing::{get, post, delete,patch},
    http::{header, HeaderValue},
    Router,
};

use mongodb::{
    options::ClientOptions,
    Client,
};

use axum::http::{
    Method
};

use tower_http::{
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
    cors::Any
};

//use tower::{ServiceBuilder, ServiceExt, Service};

mod handlers;
mod structs;

use structs::common::DatabaseConfig;

use handlers::{
    common::{handler_404, root},
    user::{create_user,delete_user,user_from_email,user_from_username,update_user},
    restaurant::{create_restaurant,restaurant_from_name,fetch_all_restaurant,fetch_restaurant_by_string},
    reviews::{create_review,get_reviews_from_restaurant},
};

use std::net::SocketAddr;





#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri).await.unwrap();
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    // the server will select the algorithm it supports from the list provided by the driver
    client_options.compressors = database_config.compressors;
    let client = Client::with_options(client_options).unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root)) // `GET /` goes to `root`
        .route("/users/create/", post(create_user)) // CREATE USER API
        .route("/users/delete/:email/", delete(delete_user)) // DELETE USER BY EMAIL API
        .route("/users/email/:email/", get(user_from_email)) // GET USER BY EMAIL API
        .route("/users/name/:name/", get(user_from_username)) // GET USER BY USERNAME API
        .route("/users/update/",patch(update_user)) // UPDATE USER API
        .route("/restaurants/all/", get(fetch_all_restaurant)) // GET ALL RESTAURANTS API
        .route("/restaurants/all/:search/", get(fetch_restaurant_by_string)) // GET RESTAURANTS BY SEARCH API
        .route("/restaurants/create/", post(create_restaurant)) // CREATE RESTAURANT API
        .route("/restaurants/:name/", get(restaurant_from_name)) // GET RESTAURANT BY NAME API
        .route("/restaurants/:name/reviews/", get(get_reviews_from_restaurant)) // GET REVIEWS FROM RESTAURANT BY NAME API
        .route("/restaurants/:name/reviews/create/", post(create_review)) // CREATE REVIEW API
        // .route("/mflix/user/login/", post(login))
        // .route("/mflix/user/signup/", post(signup))
        // set cors
        .layer(
            tower_http::cors::CorsLayer::new()
            // allow `GET`,`POST` and 'DELETE' when accessing the resource
            .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
            // allow requests from any origin
            .allow_origin(Any)
            .allow_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE, // Add this line to allow 'Content-Type'
            ])
        )

        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            HeaderValue::from_static("rust-axum"),
        ));

    let app = app.fallback(handler_404).with_state(client);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

