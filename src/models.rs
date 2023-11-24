use serde::{Deserialize, Serialize};
use std::string::String;
use hsl::HSL as HSLType;
use crate::color::random_color;
use crate::colors::COLORS;

#[derive(Deserialize, Serialize)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Deserialize, Serialize)]
pub struct HSL {
    h: f64,
    s: f64,
    l: f64,
}

#[derive(Deserialize, Serialize)]
pub struct RandomColorJSON {
    hex: String,
    name: Option<String>,
    rgb: RGB,
    hsl: HSL,
}

pub fn random_color_json() -> RandomColorJSON {
    let random_hex: String = random_color();
    let hex_string = &random_hex.get(1..).unwrap();
    let vec = hex::decode(&hex_string).unwrap();
    let rgb = RGB {
        r: vec[0],
        g: vec[1],
        b: vec[2]
    };
    let hsl_convert = HSLType::from_rgb(&vec);
    let hsl = HSL {
        h: hsl_convert.h,
        s: hsl_convert.s,
        l: hsl_convert.l
    };
    let opt = COLORS.iter().find(|&&color| &hex_string.to_uppercase() == color[0]);

    if opt.is_none() {
        create_obj(random_hex, None, rgb, hsl)
    }
    else {
        create_obj(random_hex, Option::from(opt.unwrap()[1].to_string()), rgb, hsl)
    }
}

fn create_obj(hex: String, name: Option<String>, rgb: RGB, hsl: HSL) -> RandomColorJSON {
    RandomColorJSON {
        hex,
        name,
        rgb,
        hsl
    }
}