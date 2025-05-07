#[link_section = ".custom"]
static DATA: [u8; 4] = [1, 2, 3, 4];

fn main() {
    println!("Data placed in custom section!");
}
