mod math {
    pub fn add(a: i32 , b:i32) -> i32 {a +b}

}

use math::add;

fn main() {
    println!("{}", add(2, 3));
}
