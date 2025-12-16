struct Cache<'a> {
    cached_data: &'a str,
}

impl<'a> Cache<'a> {
    pub fn get_longer<'b>(&'a self, other_data: &'b str) -> &'b str {
        if other_data.len() > self.cached_data.len() {
            other_data
        } else {
            self.cached_data
        }
    }
}

fn main() {
    let s1 = String::from("Short");
    let cache = Cache { cached_data: &s1 };

    {
        let s2 = String::from("This is a much longer ");
        let longer_ref = cache.get_longer(&s2);
        println!("Longest String {}", longer_ref);
    }
}
