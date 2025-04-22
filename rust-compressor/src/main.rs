mod rle;
mod lz;

use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: compress|decompress <input> <output> --rle|--lz");
        process::exit(1);
    }

    let mode = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];
    let algo = &args[4];

    let input = fs::read(input_path).unwrap_or_else(|_| {
        eprintln!("Failed to read input file: {}", input_path);
        process::exit(1);
    });

    let output = match (mode.as_str(), algo.as_str()) {
        ("compress", "--rle") => rle::compress_rle(&input),
        ("decompress", "--rle") => rle::decompress_rle(&input),
        ("compress", "--lz") => lz::compress_lz(&input),
        ("decompress", "--lz") => lz::decompress_lz(&input),
        _ => {
            eprintln!("Invalid mode or algorithm");
            process::exit(1);
        }
    };

    fs::write(output_path, output).unwrap_or_else(|_| {
        eprintln!("Failed to write output file: {}", output_path);
        process::exit(1);
    });

    println!("{}ion successful!", mode);
}
