macro_rules! show_path {
    ($p:path) => {
        println!("Path: {}", stringify!($p));
    };
}

fn main() {
    show_path!(std::io::Result); // Output: Path: std::io::Result
}
