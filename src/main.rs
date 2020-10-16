use html_parse;
use html_parse::parse;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::env;

fn main() {
    // let file = "examples/input";
    let input_file = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("No input file path.");
            process::exit(1);
        }
    };
    let output_file = match env::args().nth(2) {
        Some(path) => path,
        None => {
            println!("No output file path.");
            process::exit(1);
        }
    };
    let mut buffer = String::new();
    let mut input = File::open(input_file).unwrap();
    input.read_to_string(&mut buffer).unwrap();
    if buffer.is_empty() {
        eprintln!("input file missing");
        process::exit(1);
    }
    let tree = parse(&buffer);

    // println!("{:#?}", tree);
    let mut output = File::create(output_file).unwrap();
    let buffer = format!("{:#?}", tree).into_bytes();
    output.write(&buffer).expect("failed to write tree into output file.");
}
