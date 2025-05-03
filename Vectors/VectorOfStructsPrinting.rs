#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let users = vec![
        User {
            name: String::from("Alice"),
            age: 30,
        }
    ];
    
    // প্রথম উপায়: ডিবাগ প্রিন্ট
    println!("{:?}", users[0]);
    
    // দ্বিতীয় উপায়: ফর্মেটেড প্রিন্ট
    println!("User {} - Age {}", users[0].name, users[0].age);
}
