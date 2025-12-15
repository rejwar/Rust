trait Printable {
    fn PrintDetails(&self);
}

struct User {
    username: String,
}

impl Printable for User {
    fn PrintDetails(&self) {
        println!("User {}", self.username);
    }
}
fn main() {}
