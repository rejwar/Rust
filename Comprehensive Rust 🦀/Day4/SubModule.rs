mod Greetings {
    pub mod English {
        pub fn Hello() {
            println!("Hello in English!");
        }
    }

    pub mod Bangla {
        pub fn Hello() {
            println!("হ্যালো বাংলায়!");
        }
    }
}

fn main() {
    Greetings::English::Hello(); // English মডিউলের ফাংশন কল করা
    Greetings::Bangla::Hello();  // Bangla মডিউলের ফাংশন কল করা
}
