use axum::{
    routing::{get, post},
    http::{header, HeaderValue},
    Router,
};

use mongodb::{
    options::ClientOptions,
    Client,
};

use tower_http::{
    limit::RequestBodyLimitLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer
};

mod handlers;
mod structs;

use structs::common::DatabaseConfig;
use handlers::{
    common::{handler_404, root},
    user::create_user,
    mflix::{list_users, user_by_id, user_by_name, user_by_email},
    //auth::{login, signup}
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
        .route("/users", post(create_user)) // `POST /users` goes to `create_user`
        .route("/mflix/users/", get(list_users))
        .route("/mflix/user/id/:id/", get(user_by_id))
        .route("/mflix/user/name/:name/", get(user_by_name))
        .route("/mflix/user/email/:email/", get(user_by_email))
        //.route("/mflix/user/login/", post(login))
        //.route("/mflix/user/signup/", post(signup))
        
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

