//Testing env::var
// add `mod env_var` and call `env_var::run()` into src/main.rs
// Running learning-rust without setting ENV_VAR exits unsucessfully
// and warns that variable isn't present
// To setup ENV_VAR
// $ ENV_VAR=123 cargo run
// or $ ENV_VAR=abc ./target/debug/learning-rust
// issue $ echo $? immediatly after running learning-rust to check proccess exit code.

// 1. fixing error: use of undeclared crate or module `env`
use std::{
    env::var
};

// 2. fixing error: ENV_VAR not found in this scope
const ENV_VAR: &str = "ENV_VAR";

pub fn run() {
    // 3. unfixed error: cannot infer type for type parameter `T` declared on the trait
    // commenting out .into() fixed this error
    // function into seems to convert string to array of bytes     
    var(ENV_VAR).expect("missing ENV_VAR");//.into();
}