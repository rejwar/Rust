fn main() {
    let special_tpl: () = ();
    let mut tpl = (500, "Ho",true);

    println!("{:.?}", tpl);

    let (x , y ,z) = tpl;

    println!("{}",x);
    println!("{}", y);
    println!("{}", z);
}