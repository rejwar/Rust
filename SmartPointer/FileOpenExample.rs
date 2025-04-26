use std::fs::File;
fn main() {
    let File = File::open ("Example.txt");
    match File{

        Ok(file_handle) => println!("File opened successfully"),
        Err(error) => println!("Error Opening file: {:?}", error),
        
    }
}
