use std::fs::Permissions;

pub trait Describable  {
    fn Describe(&self) -> String;
}

struct Product {
    Name: String,
}

impl Describable for Product {
    fn Describe(&self) -> String {
        format!("Product {}" , self.Name)
    }
    
}

fn main() {
    let Item = Product {Name: String::from("Laptop")};
    println!("{}" , Item.Describe());
}
