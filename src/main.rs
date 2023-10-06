#![allow(unused)]

use std::net::{SocketAddr, Ipv4Addr, IpAddr};

use axum::Router;
use axum::extract::Query;
use axum::routing::get;
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello)
    );
    
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr).serve(routes_hello.into_make_service()).await.unwrap();
}

// region: --- Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");
    // as deref that will give option of reference of string this way we wont have any new String allocation.
    let name = params.name.as_deref().unwrap_or("meow");
    Html(format!("Hello <strong>{name}</strong>"))
}