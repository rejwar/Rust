fn main() {
    let company = String::from("Mike Dex");

    let print_company = move || {
        println!(" Inside closure {}", company);
    };

    print_company();

    println!("  ")
}
