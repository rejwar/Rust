
let shapes : Vec<Box<dyn Shape >> = vec![
    Box::new(Circle {radius: 5.0}),
    Box::new(Rectangle {width: 10.0 , height: 4.0}),
];

for shape in shapes {
    println!("Area is {}", shape.Area());
}