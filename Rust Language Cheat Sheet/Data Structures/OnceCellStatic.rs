use once_cell::sync::Lazy;

static VERSION: Lazy<String> = Lazy::new(||{
    println!("Version Static only created one time");
    "V>!!".to_string()
});

fn main() {
    println!("FIRst access {}", *VERSION);
    println!("2nd access {}", *VERSION);
}
