use weezl::{BitOrder, encode::Encoder};
use base64::{Engine as _, engine::general_purpose};

pub fn compress_str(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let compressed = Encoder::new(BitOrder::Msb, 9)
        .encode(input.as_bytes())
        .unwrap();
    Ok(general_purpose::STANDARD_NO_PAD.encode(&compressed))
}