// Question: How can we use tuple structs and the newtype pattern in Rust?

struct UserId(u32); // newtype pattern
struct Point(i32, i32); // tuple struct

fn UseNewtypeAndTupleStruct() {
    let user_id = UserId(1001);
    let point = Point(10, 20);

    println!("User ID: {}", user_id.0);
    println!("Point: ({}, {})", point.0, point.1);
}
