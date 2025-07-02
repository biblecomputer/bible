use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("stv_compressed.bin");

    // Read the JSON file
    let json_content = fs::read_to_string("src/stv.json")
        .expect("Failed to read stv.json");

    // Compress it with maximum compression
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(json_content.as_bytes())
        .expect("Failed to compress data");
    let compressed_data = encoder.finish()
        .expect("Failed to finish compression");

    // Write compressed data to output directory
    fs::write(&dest_path, &compressed_data)
        .expect("Failed to write compressed data");

    println!("cargo:rerun-if-changed=src/stv.json");
    
    // Print compression stats
    let original_size = json_content.len();
    let compressed_size = compressed_data.len();
    let ratio = (compressed_size as f64 / original_size as f64) * 100.0;
    
    println!("cargo:warning=Bible JSON compressed: {} bytes -> {} bytes ({:.1}% of original)", 
             original_size, compressed_size, ratio);
}