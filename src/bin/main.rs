use std::fs::File;
use std::io;
use std::io::prelude::*;

use clap::{App, Arg};

use rot13::{rot13, Mode};

fn main() {
    // parse arguments
    let matches = App::new("rot13")
        .version("0.1")
        .author("Cameron Phillips. <cameron0505@gmail.com>")
        .about("rot-13 encryption and decryption")
        .arg(
            Arg::with_name("encrypt")
                .help("encrypt the provided input")
                .short("e")
                .long("encrypt")
                .conflicts_with("decrypt")
                .required(true),
        )
        .arg(
            Arg::with_name("decrypt")
                .help("decrypt the provided input")
                .short("d")
                .long("decrypt")
                .conflicts_with("encrypt")
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("the input file to use. defaults to stdin")
                .short("i")
                .long("input")
                .value_name("PATH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("the output file to use. defaults to stdout")
                .short("o")
                .long("output")
                .value_name("PATH")
                .takes_value(true),
        )
        .get_matches();

    // open the input file if the argument is set, otherwise use stdin
    let mut input: Box<dyn Read> = if let Some(path) = matches.value_of("input") {
        Box::new(File::open(path).expect(&format!("failed to open input file {}", path)))
    } else {
        Box::new(io::stdin())
    };

    // open the output file if the argument is set, otherwise use stdout
    let mut output: Box<dyn Write> = if let Some(path) = matches.value_of("output") {
        Box::new(File::create(path).expect(&format!("failed to open output file {}", path)))
    } else {
        Box::new(io::stdout())
    };

    // determine whether we are encrypting or decrypting
    let mode = if matches.is_present("encrypt") {
        Mode::Encrypt
    } else {
        Mode::Decrypt
    };

    // copy the input to the output through a rot13 encoder
    match rot13(mode, &mut input, &mut output) {
        Ok(()) => {},
        Err(e) => eprintln!("error: {:?}", e),
    }
}
