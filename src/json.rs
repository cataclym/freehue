use serde::{Deserialize, Serialize};
use crate::color::random_color;

#[derive(Deserialize, Serialize)]
pub struct RandomColorJSON {
    color: String,
}

pub fn random_color_json() -> RandomColorJSON {
    RandomColorJSON {
        color: random_color(),
    }
}