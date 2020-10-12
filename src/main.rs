use html_parse;
use html_parse::parse;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::env;

fn main() {
    // let file = "examples/input";
    let file = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Please input file path.");
            process::exit(1);
        }
    };
    let mut buffer = String::new();
    let mut file = File::open(file).unwrap();
    file.read_to_string(&mut buffer).unwrap();
    if buffer.is_empty() {
        eprintln!("input file missing");
        process::exit(1);
    }
    let tree = parse(&buffer);

    println!("{:#?}", tree);
}
