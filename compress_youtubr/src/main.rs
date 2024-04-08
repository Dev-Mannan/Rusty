extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    println!("This is a YouTube compression program built with Rust!");
    println!();
    let arguments: Vec<String> = args().collect();
    if arguments.len() != 3 {
        eprintln!("Usage: 'source' 'target'");
        return;
    }
    let input_file = File::open(&arguments[1]).unwrap();
    let output_file = File::create(&arguments[2]).unwrap();

    let mut input = BufReader::new(input_file);
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let _ = encoder.finish().unwrap();

    println!("Elapsed: {:?}", start.elapsed());
}
