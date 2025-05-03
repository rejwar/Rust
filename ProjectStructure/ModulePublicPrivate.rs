mod secret() {
    fn HiddenMessage() {
        println!("This is Private");
    }

    pub fn ShowMessage() {
        println!("This is public");
    }
}


fn main() {
    secret::ShowMessage();
}
