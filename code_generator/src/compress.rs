use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::prelude::*;
use std::io::{Read, Write};
use base64::{Engine as _, engine::general_purpose};

fn main() {
    let input_string = r#"
<path d="M20 22h-2"></path>
<path d="M20 15v2h-2"></path>
<path d="M4 19.5V15"></path>
<path d="M20 8v3"></path>
<path d="M18 2h2v2"></path>
<path d="M4 11V9"></path>
<path d="M12 2h2"></path>
<path d="M12 22h2"></path>
<path d="M12 17h2"></path>
<path d="M8 22H6.5a2.5 2.5 0 0 1 0-5H8"></path>
<path d="M4 5v-.5A2.5 2.5 0 0 1 6.5 2H8"></path>"#;

    // compress string
    // let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    // encoder.write_all(input_string.as_bytes()).unwrap();
    // let compressed_bytes = encoder.finish().unwrap();
    //
    // // decompress string
    // let mut decoder = GzDecoder::new(&compressed_bytes[..]);
    // let mut decompressed_string = String::new();
    // decoder.read_to_string(&mut decompressed_string).unwrap();
    //
    // println!("Original string: {}", input_string);
    // println!("Compressed bytes({}): {:?}", compressed_bytes.len(), compressed_bytes);
    // println!("Decompressed string({}): {}", decompressed_string.len(), decompressed_string);



    let compressed = general_purpose::STANDARD_NO_PAD.encode(input_string);
    let decompressed = &general_purpose::STANDARD_NO_PAD.decode(compressed.clone()).unwrap();


    println!("Original string: {}", input_string.len());
    println!("Compressed bytes({}): ", compressed.clone().len() );
    println!("Decompressed string({})", decompressed.len());

}