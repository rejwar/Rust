fn main() {
    let s = String::from("hello");
    let References = &s;

    println!("Length (explicit dereferning ) :{}", (*References).len());
    println!("Length (auto - dereferning ) {}", References.len());

    struct Point{
        x : i32 ,
        y : i32 ,
    }

    let p = Point {x : 1 , y: 2};
    let ReferenceP = &p;

    println!("Point x (explicit dereferencing ) :{}", (*ReferenceP).x);
    println!("Point y (auto deferencing : {})", ReferenceP.y);
}
