fn main() {
    let mut s = String::from("Hello"); // ১. ভেরিয়েবল মিউটেবল

    change(&mut s); // ২. আমরা &mut পাঠিয়েছি

    println!("{}", s); // আউটপুট হবে: Hello, World!
}

fn change(text: &mut String) {
    text.push_str(", World!"); // আমরা অরিজিনাল স্ট্রিংটি পরিবর্তন করলাম
}
