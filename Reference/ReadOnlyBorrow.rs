fn main() {
    let Title: String = String::from("Rust Book");

    let R1 = &Title;
    let R2 = &Title;


    PrintReaders(R1 , R2);
    println!("Owner -> {}", Title);

}

fn PrintReaders(A: &String , B: &String) {
    println!("Readers -> {} {}", A,B);
}