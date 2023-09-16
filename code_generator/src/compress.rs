use flate2::write::ZlibEncoder;

use flate2::Compression;
use std::io::prelude::*;


pub fn compress_string(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    encoder.write_all(input.as_bytes())?;

    let compressed_bytes = encoder.finish()?;

    Ok(base64::encode(&compressed_bytes))
}

// pub fn decompress_string(input: &str) -> String {
//
//     use flate2::read::ZlibDecoder;
//     use std::io::prelude::*;
//
//
//     let input = base64::decode(input).unwrap();
//
//     let mut decoder = ZlibDecoder::new(input.as_slice());
//
//     let mut decompressed = String::new();
//
//     decoder.read_to_string(&mut decompressed).expect("decompress");
//
//     decompressed
// }
//
// fn main() {
//     let input = r#"
// <path d="M20 22h-2"></path>
// <path d="M20 15v2h-2"></path>
// <path d="M4 19.5V15"></path>
// <path d="M20 8v3"></path>
// <path d="M18 2h2v2"></path>
// <path d="M4 11V9"></path>
// <path d="M12 2h2"></path>
// <path d="M12 22h2"></path>
// <path d="M12 17h2"></path>
// <path d="M8 22H6.5a2.5 2.5 0 0 1 0-5H8"></path>
// <path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8"></path>"#;
//
//     let compressed_str = compress_string(input).unwrap();
//     let decompressed_str = decompress_string(compressed_str.as_str());
//
//     println!("Original string size {}", input.len());
//     println!("Compressed string: {}", compressed_str);
//     println!("Compressed string size {}", compressed_str.len());
//
//     println!("Decompressed string: {}", decompressed_str);
//     println!("Decompressed string size {}", decompressed_str.len());
//
// }