
pub mod parser;

use std::{
    io::Read,
    fs,
    sync::Arc
};

fn main() {

    let file_path = "Exemplo/pagina.html";
    let mut file = fs::File::open(file_path).expect("Failed to open file");

    let mut html_contents = String::new();

    file.read_to_string(&mut html_contents).expect("Failed to read file");

    let html: Arc<String> = Arc::new(html_contents.clone());

    let tags: Vec<String> = vec![
        "link".to_string(),
        "img".to_string(),
        "script".to_string()
    ];

    parser::tag_handler(html, tags);
}
