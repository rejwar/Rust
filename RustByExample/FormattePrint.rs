fn main() {
    println!("{} days " , 31);
    println!("{} , This is {1}.{1} , this is {0}" , "Alice" , "Bob"   );

    println!("{subject} {verb} {object}" ,
            object= "The lazy dog",
            subject= "THe quick brown FOx",
            verb = "Jumps Over");
}

println!("Base 10:  {}" , 69420);
println!("Base 2  (binary) {:b}" , 69420);
println!("Base 8 (octal) :  {:o}" , 69420);
println!("Base 16 (hexadecimal) : {:h}" , 69420);


println!("{Numbers:> 5}" , numbers = 1);
println!("{numbers : 0>5}" , numbers = 1);
println!("{numbers : 0< 5}" , numbers =1 );
println!("{numbers : 0> width$ }" , numbers = 1);
println!("My name is {0} , {1} , {0}" , "Bond");


#[allow(dead_code)]
struct Structure(i32);

let number: f64 = 1.0;
let width: usize = 5;
println!("{numbers :> width}");
