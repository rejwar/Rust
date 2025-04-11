struct Product {
    Name: String,
}

impl Product {
    fn new(Name: String) -> Product {
        Product { Name }
    }
    fn display(self) {
        println!("Product Name: {}", self.Name);
    }
}

fn main() {
    let MyProduct: Product = Product::new(String::from("Laptop"));
    MyProduct.display(); // Ownership used and ends
}
