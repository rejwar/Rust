fn main() {
    let s1: &str = "World";
    println!("S1 : {}", s1);

    let mut s2: String = String::from("Hellp");
    println!("S2 : {}", s2);

    s2.push_str(s1);
    println!("S2 {}" ,s2 );

    let s3: &str = &s2[2..4];
    println!("S3 : {}", s3);
}
