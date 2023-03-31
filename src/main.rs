use axum::{
    routing::{get, post},
    http::{header, HeaderValue},
    //response::IntoResponse,
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
    handlers::create_user,
};

use std::net::SocketAddr;
use std::time::Duration;

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
        // timeout requests after 10 seconds, returning a 408 status code
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        //do not allow request bodies larger than 1024 bytes, returning 413 status code
        .layer(RequestBodyLimitLayer::new(1024))
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

