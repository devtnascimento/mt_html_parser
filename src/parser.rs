use std::{
    thread,
    sync::Arc
};
use scraper::{Html, Selector};


fn get_tag(html: Arc<String>, tag: String) {

    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse(tag.as_str())
        .expect("Failed to parse selector");

    let specific_attributes = ["href", "src"];

    for element in document.select(&selector) {
        let tag_name = element.value().name();
        for attr in &specific_attributes {
            if let Some(attr_value) = element.value().attr(*attr) {
                println!("Tag Name: {}", tag_name);
                println!("Attribute Name: {}", attr);
                println!("Attribute Value: {}", attr_value);
            }
        }
    }
}

pub fn tag_handler(html: Arc<String>, tags: Vec<String>){
    let mut handles = Vec::new();
    for tag in tags {
        let html_ref = Arc::clone(&html);
        let handle = thread::spawn(move||{
            get_tag(html_ref, tag);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
