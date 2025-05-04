fn unwrap_and_expect() {
    let value = Some("Rust");

    // unwrap() ব্যবহার - Some হলে ভ্যালু রিটার্ন করে, None হলে panic করে
    println!("Using unwrap(): {}", value.unwrap());
    
    // expect() ব্যবহার - Some হলে ভ্যালু রিটার্ন করে, None হলে কাস্টম মেসেজসহ panic করে
    println!("Using expect(): {}", value.expect("No value found"));
    
    // নিরাপদ উপায় - pattern matching ব্যবহার
    match value {
        Some(v) => println!("Safely extracted: {}", v),
        None => println!("No value present"),
    }
}
