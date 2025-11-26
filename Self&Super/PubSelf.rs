mod fruit {
    pub fn apple() {
        println!("This is apple");
    }

    pub fn show() {
        // এখানে self keyword ব্যবহার করে apple function call করুন
        self::apple();
    }
}

fn main() {
    fruit::show();
}
