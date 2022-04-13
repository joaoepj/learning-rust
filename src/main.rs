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
mod cli;
mod cfg_from_file;


use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use clap::Parser;









fn main() {
    let cli = cli::Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        cli::Commands::CfgFromToml => {
            cfg_from_file::read_from_file("../../src/library.example".into())
        }
        cli::Commands::EnvVar => {
           println!("{:#?}", env_var::unwrap("ENV_VAR"));
        }
        cli::Commands::CodecDemo => {
           codec_demo::run();
        }
        cli::Commands::Assert => {
           println!("Assert: {:#?}",assert_test::run());
        }
        cli::Commands::TokioMrCli => {
           let mut rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                    println!("Aqui é tokio rapah!!!");
                })
        }
        cli::Commands::TokioMrSrv => {
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