fn print_str (s: &str) 
{
    println!("{}", s);
}

fn main() {
    let s:String = String:: from("Rust");
    print_str(&s);
}
