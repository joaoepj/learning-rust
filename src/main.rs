//mod url_host;
mod env_var;

fn main() {
    //url_host::run();
    //env_var::run();
    //very disapointinh I really give up of this if (at lest for now)
    // Why this fucking unit `()` always appears?
    // I need to elegantly get rid of it
        println!("{:#?}", env_var::unwrap("ENV_VAR"));
    
    
}
