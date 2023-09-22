extern crate core;


use base64::*;
use base64::{ engine::general_purpose};
use lucide_icon_data::LucideIcon;
use strum::EnumProperty;
use weezl::{BitOrder, decode::Decoder};

use crate::lucide_icon_data;

fn decompress_str(input: &str) -> String {
    let compressed = general_purpose::STANDARD_NO_PAD.decode(input).unwrap();
    let decompressed = Decoder::new(BitOrder::Msb, 9)
        .decode(&compressed.to_vec())
        .unwrap();
    return String::from_utf8(decompressed).unwrap();
}

impl LucideIcon {

    pub fn svg(&self) -> String {
        decompress_str(self.get_str("svg").expect("get svg"))
    }

    pub fn categories(&self) -> Vec<&str> {
        self.get_str("categories")
            .expect("get categories")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn tags(&self) -> Vec<&str> {
        self.get_str("tags")
            .expect("get tags")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn contributors(&self) -> Vec<&str> {
        self.get_str("contributors")
            .expect("get contributors")
            .split(',')
            .collect::<Vec<&str>>()
    }

    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}