fn borrows(s: &String) {
    println!("{}", s);
} // এখানে শুধু reference ব্যবহার হয়, ownership move হয় না

fn main() {
    let s1 = String::from("hello");
    borrows(&s1); // borrow করা হলো
    println!("{}", s1); // ✅ এখনও ব্যবহারযোগ্য
}
