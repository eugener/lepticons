extern crate core;

use core::fmt;
// use fmt::Result;


use crate::lucide_icon_data;
use lucide_icon_data::LucideIcon;

use base64::*;
use flate2::read::ZlibDecoder;

use std::io::prelude::*;
use strum::EnumProperty;


fn decompress(input: &str) -> String {
    let input = base64::decode(input).unwrap();
    let mut decoder = ZlibDecoder::new(input.as_slice());
    let mut decompressed = String::new();
    decoder
        .read_to_string(&mut decompressed)
        .expect("decompress");
    decompressed
}

impl LucideIcon {


    pub fn svg(&self) -> String {
        decompress(self.get_str("svg").expect("get svg"))
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