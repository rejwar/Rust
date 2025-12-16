fn main() {
    let shapes: Vec<Box<dyn shape>> =
        vec![Box::new(Circle { r: 3.0 }), Box::new(Square { s: 2.0 })];

    for s in shapes {
        println!("{}", s.area)
    }
}
