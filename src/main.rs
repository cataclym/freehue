mod html;
mod color;

use warp::{Filter};
use crate::html::create_html_body;
use crate::color::random_color;

#[tokio::main]
async fn main() {
    println!("Starting freehue");

    let random = warp::path!("random")
        .map(|| warp::reply::html(create_html_body("title", format!("#{}", random_color()))));

    warp::serve(random)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

