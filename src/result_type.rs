use std::path::PathBuf;
use std::io::{Error, Read};
use std::fs::File;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    // Return a File given input PathBuf
    // let mut file: File = match File::open(path) {
    //     Ok(file_handle) => file_handle,
    //     Err(error) => return Err(error),
    // };

    // Alternative implementation with ?
    let mut file: File = File::open(path)?;

    // Read file contents into `string`` variable with `read_to_string`
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(error) => return Err(error),
    };

    // Return `string` variable as expected by function signature
    Ok(string)
}

fn main() {
    if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
        println!("The program found the main file.");
    }
    if read_file_contents(PathBuf::from("non_existent_file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}