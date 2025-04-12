struct Website {
    IsAvailable: bool,
    HasSSL: bool,
}

impl Website {
    fn IsSecure(&self) -> bool {
        self.IsAvailable && self.HasSSL
    }
}

fn main() {
    let MyWebsite: Website = Website {
        IsAvailable: true,
        HasSSL: true,
    };

    let IsSecure: bool = MyWebsite.IsSecure();
    println!("Is Website Secure? {}", IsSecure);
}
