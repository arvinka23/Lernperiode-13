extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("usage: source target");
        return;
    }
    let source = args().nth(1).unwrap();
    let target = args().nth(2).unwrap();
    let input_file = File::open(&source).unwrap();
    let input_size = input_file.metadata().unwrap().len();
    let mut input = BufReader::new(input_file);
    let output = BufWriter::new(File::create(&target).unwrap());
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output_file = encoder.finish().unwrap();
    let output_size = output_file.get_ref().metadata().unwrap().len();
    let elapsed = start.elapsed();
    println!("compressed {} to {} in {:?}", source, target, elapsed);
    println!("{} bytes to {} bytes: {:.2}%", input_size, output_size, output_size as f64 / input_size as f64 * 100.0);
}
