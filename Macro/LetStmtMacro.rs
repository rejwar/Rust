macro_rules! declare_and_use {
    ($s:stmt) => {
        $s
        println!("Declared!");
    };
}

fn main() {
    declare_and_use!(let name = "Rust";);
}
