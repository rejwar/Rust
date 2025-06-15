struct Important<'a>
{
    data: &'a str,
}

fn main() {
    let text = String::from("Rust is awesome");
    let imp = Important {data : &text};
    println!("{}", imp.data);

}
