#![feature(iterator_for_each)]

extern crate id3;
extern crate encoding;
extern crate clap;

use encoding::{EncodingRef};
use clap::{Arg, App, SubCommand};

use std::iter::FromIterator;

mod change_encoding;

fn main() {
    let matches = App::new("rusty id3 tool")
        .version("0.1")
        .author("Alex ")
        .about("Manipulates id3 metadata")
        .subcommand(SubCommand::with_name("encoding")
            .about("Fixes id3 encoding")
            .version("0.1")
            .author("Alex ")
            .arg(Arg::with_name("ENCODING")
                .help("Sets the encoding")
                .required(true)
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["FROM", "TO"])
                .index(1))
            .arg(Arg::with_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .index(2)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("encoding") {
        let file_name = matches.value_of("FILE").unwrap();

        let (from, to) = matches.values_of("ENCODING")
            .map(Vec::from_iter)
            .map(|encoding_names| (encoding_names[0], encoding_names[1]))
            .map(|(from, to)| (encoding(from), encoding(to)))
            .unwrap();

        change_encoding::change(file_name, from, to);
    }
}

fn encoding(encoding_name: &str) -> EncodingRef {
    encoding::label::encoding_from_whatwg_label(encoding_name)
        .expect(&format!("Unknown encoding: {}", encoding_name))
}
