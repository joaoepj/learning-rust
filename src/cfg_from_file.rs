use serde::Deserialize;
use std::fs;

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


pub fn read_from_file(path: String ) {
    let content = fs::read_to_string(path)
                .expect("Something went wrong reading the file");
            let library: Library = toml::from_str(content.as_str()).unwrap();
            println!("{:#?}", library);
}