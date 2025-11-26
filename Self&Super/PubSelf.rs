mod fruit {
    pub fn apple() {
        println!("This is apple");
    }

    pub fn show() {
        self::apple();
    }
}

fn main() {
    fruit::show();
}
