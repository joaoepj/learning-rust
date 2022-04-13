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

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};


use serde::Deserialize;
use std::fs;
use clap::{Parser, Subcommand};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    //
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug,Subcommand)]
enum Commands {
    /// Read config from toml file
    CfgFromToml,
    /// Read config from environment variables
    EnvVar,
    /// Test Parity Scale CODEC
    CodecDemo,
    /// Test assert
    Assert,
    /// Mini redis client built with tokio async library
    TokioMrCli,
    /// Mini redis server built with tokio async library
    TokioMrSrv
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
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::CfgFromToml => {
            let content = fs::read_to_string("../../src/library.example")
                .expect("Something went wrong reading the file");
            let library: Library = toml::from_str(content.as_str()).unwrap();
            println!("{:#?}", library);
        }
        Commands::EnvVar => {
           println!("{:#?}", env_var::unwrap("ENV_VAR"));
        }
        Commands::CodecDemo => {
           codec_demo::run();
        }
        Commands::Assert => {
           println!("Assert: {:#?}",assert_test::run());
        }
        Commands::TokioMrCli => {
           let mut rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                    println!("Aqui é tokio rapah!!!");
                })
        }
        Commands::TokioMrSrv => {
           let mut rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

                        loop {
                            // The second item contains the IP and port of the new connection.
                            let (socket, _) = listener.accept().await.unwrap();
                            process(socket).await;
                        }
                    })
        }
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}