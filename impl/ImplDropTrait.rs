struct File;

impl Drop for File  {
    fn drop(&mut self) {
        println!("File closed ");
    }
}