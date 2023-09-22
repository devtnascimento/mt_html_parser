
mod parser;
mod http;

#[allow(unused)]

use std::{
    io::{Read, Write},
    net::TcpStream,
    fs,
    sync::Arc,
    rc::Rc
};
use http::request::HttpClient; //modulo de request implementado em ./http/request.rs
use http::request::Http;

fn main() -> std::io::Result<()> {

    let mut client = HttpClient::new(Rc::new("127.0.0.1:8080".to_string()));

    let response = client.get(None)?;

    println!("Response:\n{}", response);

    let html_contents = response.to_string();

    let html: Arc<String> = Arc::new(html_contents);

    let tags: Vec<String> = vec![
        "link".to_string(),
        "img".to_string(),
        "script".to_string()
    ];

    parser::tag_handler(html, tags);

    Ok(())

}

