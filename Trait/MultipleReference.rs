struct Pair<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Pair<'a> {
    fn longest(&self) -> &str {
        if self.left.len() >= self.right.len() {
            self.left
        } else {
            self.right
        }
    }
}

fn main() {
    let a = String::from("Short");
    let b = String::from("Much longer ");
    let p = Pair {
        left: &a,
        right: &a,
    };

    println!("{} ", p.longest());
}
