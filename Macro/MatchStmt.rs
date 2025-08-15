macro_rules! match_wrapper {
    ($s:stmt) => {
        println!("Matching...");
        $s
    };
}

fn main() {
    match_wrapper!(match 1 {
        1 => println!("One"),
        _ => println!("Other"),
    });
}
