fn main() {
     let SomeOption = Some(12);

     if let Some(x) = SomeOption {
         println!("The value is: {}", x);
     } else {
         println!("No value found");
     }
     println!("End of program");
}
