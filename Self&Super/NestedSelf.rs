mod shape {
    pub fn name() {
        println!("I am a shape");
    }
    pub fn show() {
        self::name();
    }
}

fn main() {
    shape::show();
}
