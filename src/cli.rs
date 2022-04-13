use clap::{Parser, Subcommand};

/// Simple program to learning-rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    //
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug,Subcommand)]
pub enum Commands {
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

