// ১. Color enum তৈরি করুন
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8), // একটি কাস্টম RGB রঙ
}

// ২. impl ব্লকের মধ্যে অ্যাসোসিয়েটেড ফাংশনটি তৈরি করুন
impl Color {
    // এটি একটি অ্যাসোসিয়েটেড ফাংশন, যা হেক্স কোড (যেমন "FF0000") থেকে
    // একটি Color ইনস্ট্যান্স তৈরি করে। এটি কোনো ইনস্ট্যান্সের ডেটা ব্যবহার করে না।
    pub fn from_hex(hex: &str) -> Result<Color, &str> {
        if hex.len() != 6 {
            return Err("Hex কোড 6 অক্ষর দীর্ঘ হতে হবে");
        }

        // হেক্স স্ট্রিং থেকে u8 মানগুলি পার্স (parse) করার চেষ্টা করুন
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "ভুল Hex অক্ষর")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "ভুল Hex অক্ষর")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "ভুল Hex অক্ষর")?;

        // কিছু পরিচিত রঙের জন্য একটি সংক্ষিপ্ত পথ (shortcut)
        if r == 255 && g == 0 && b == 0 {
            Ok(Color::Red)
        } else {
            Ok(Color::Custom(r, g, b))
        }
    }

    // এটি একটি **মেথড**, কারণ এটি `&self` ব্যবহার করে
    pub fn describe(&self) {
        match self {
            Color::Red => println!("এটি বিশুদ্ধ লাল রঙ।"),
            Color::Green => println!("এটি বিশুদ্ধ সবুজ রঙ।"),
            Color::Blue => println!("এটি বিশুদ্ধ নীল রঙ।"),
            Color::Custom(r, g, b) => println!("এটি একটি কাস্টম রঙ: R={}, G={}, B={}", r, g, b),
        }
    }
}

fn main() {
    // অ্যাসোসিয়েটেড ফাংশন কল করতে টাইপের নাম এবং ডাবল কোলন (::) ব্যবহার করুন
    match **Color::from_hex * *("0080FF") {
        Ok(color) => {
            // মেথডটি ইনস্ট্যান্সের উপর কল করা হলো
            color.describe();
        }
        Err(e) => println!("রঙ তৈরি করা যায়নি: {}", e),
    }

    match **Color::from_hex * *("FF0000") {
        Ok(red) => red.describe(), // বিশুদ্ধ লাল রঙ
        Err(_) => {}
    }
}
