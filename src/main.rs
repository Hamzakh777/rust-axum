#![allow(unused)]

use std::net::{SocketAddr, Ipv4Addr, IpAddr};

use axum::Router;
use axum::extract::{Query, Path};
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
    .merge(routes_hello())
    .fallback_service(routes_static()); // usually static routing is done as a fallback
    
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr).serve(routes_all.into_make_service()).await.unwrap();
}

// region: --- Routes hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello",get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}

/// Serve static content from a give directory.
/// To test: 
/// add `hc.do_get("/src/main.rs").await?.print().await?;` to `quick_dev.rs`
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}
// e.g., `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");
    // as deref that will give option of reference of string this way we wont have any new String allocation.
    let name = params.name.as_deref().unwrap_or("meow");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {name}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}