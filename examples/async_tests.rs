use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use std::thread;
use std::time::Duration;

async fn read_file(file_path: &str) -> io::Result<String> {
    // Open the file
    let mut file = File::open(file_path).await?;

    // Read the file's contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    if file_path == "file1.txt"{
        thread::sleep(Duration::from_secs(1));
    }
    else if file_path == "file2.txt" {
        thread::sleep(Duration::from_secs(2));
    }
    

    Ok(contents)
}

#[tokio::main]
async fn main(){

    // List of file paths to read concurrently
    let file_paths = vec!["examples/file1.txt", "examples/file2.txt", "examples/file3.txt"];


    let mut results = Vec::new();
    for i in 1..10 {
        
        thread::sleep(Duration::from_secs(1));

        println!("Lendo arquivos...");
        for file_path in &file_paths {
            let result = tokio::spawn(read_file(file_path));
            if i == 1 {results.push(result);}
            for r in results {
                println!("{} => status: {:?} ", file_path, r);
            }
            
        }
        println!("enquanto isso... ");
        println!("Task {i} concluida!");
    }

    for result in results {
        match result.await {
            Ok(data) => println!("File contents: {:?}", data),
            Err(err) => eprintln!("Error reading file: {}", err),
        }
    }







}
