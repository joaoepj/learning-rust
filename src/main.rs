/*
mod url_host;
    //url_host::run();
    //env_var::run();

    The idea is organize the code into modules
    to be called by CLI switches or provide
    service to each other.

*/
mod env_var;

use serde::Deserialize;


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

    
    let library: Library = toml::from_str(r#"
        [[books]]
        title = "B"
        author = "A"
        edition = "1ª"
        pages = 2022
        [[books]]
        title = "C"
        author = "D"
      
        "#).unwrap();

        println!("{:#?}", library);
        
        println!("{:#?}", env_var::unwrap("ENV_VAR"));
        
    
    
}
