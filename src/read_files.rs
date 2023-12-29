
use std::{env, fs::File};
use std::io::{BufRead, BufReader};

// cargo run --bin read_files src/file.txt 
// to specify argument src/file.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is "target/debug/read_files" for some reason
    println!("The first arguement is {}", args[0]);

    let file = File::open(&args[1]);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}