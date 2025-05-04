

trait Describle {
    fn Describe (&self) -> String;
}

struct Product {
    Name: String,
}

impl Describle for Product {
    fn Describe (&self) -> String {
        format!("Product Name : {}" , self.Name)
    }
}

fn main() {
    let Item = Product { Name: String::from("Laptop")};
    println!("{}" , Item.Describe());
}
