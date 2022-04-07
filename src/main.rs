/*
mod url_host;
    //url_host::run();
    //env_var::run();

    The idea is organize the code into modules
    to be called by CLI switches or provide
    service to each other.

*/
mod env_var;
mod assert_test;
mod codec_demo;

use serde::Deserialize;
use std::fs;
use clap::{Parser, Subcommand};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}



#[derive(Debug,Deserialize)]
struct Library {
    books: Vec<Books>,
}

#[derive(Debug,Deserialize)]
struct Books {
    title: String,
    author: String,
    edition: Option<String>,
    pages: Option<i16>,
}


fn main() {
    let args = Args::parse();
    //match args.name {
    //    "codec-"
    //}

    codec_demo::run();
    let content = fs::read_to_string("../../src/library.example")
        .expect("Something went wrong reading the file");
    let library: Library = toml::from_str(content.as_str()).unwrap();
    println!("{:#?}", library);
    println!("{:#?}", env_var::unwrap("ENV_VAR"));
    println!("Assert: {:#?}",assert_test::run());
}
