fn main() {
    let fullname = "TutorialsPoint \r\n";
    println!("Before trim");
    println!("Length is {}" ,fullname.len());
    println!();
    println!("After trim");
    println!("Length is {}" , fullname.trim().len());
}
