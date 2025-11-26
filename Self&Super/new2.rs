mod fruits {

    pub fn apple() {
        println!("This is apple ");
    }
    pub fn show() {
        self::apple();
    }
}

fn main() {
    fruits::show();
}
