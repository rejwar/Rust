fn CalculateLength(S: &String) -> usize {
    S.len()
} // S-এর মালিকানা এখানে নেওয়া হয়নি

fn main() {
    let S1 = String::from("hello");
    let Length = CalculateLength(&S1); // S1-কে ধার দেওয়া হলো
    println!("The length of '{}' is {}.", S1, Length); // S1 এখানেও ব্যবহারযোগ্য
}
