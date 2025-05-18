fn main() {
    let mut number = 10;
    {
        let r1 = &number;
        println!("{}",r1);
    }

    {
        let r2 = &mut number;
        *r2 += 5;
        println!("{r2}");
    }
}
