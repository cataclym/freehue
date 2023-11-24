mod html;
mod color;
mod models;
mod colors;

use warp::{Buf, Filter};
use crate::models::random_color_json;

#[tokio::main]
async fn main() {
    println!("Starting freehue");

    let get_endpoints = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("random").map(|| warp::reply::json(&random_color_json())))
        .and(warp::path::end());

    let routes = get_endpoints;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

