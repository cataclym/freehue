mod html;
mod color;
mod json;
use warp::{Filter};
use crate::json::random_color_json;

#[tokio::main]
async fn main() {
    println!("Starting freehue");
    let random = warp::path!("random")
        .map(|| warp::reply::json(&random_color_json()));

    warp::serve(random)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

