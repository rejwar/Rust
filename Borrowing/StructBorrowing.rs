struct Product {
    Name: String,
}

fn DisplayProduct(Product: &Product) {
    println!("Product Name: {}", Product.Name);
}

fn main() {
    let MyProduct: Product = Product {
        Name: String::from("Laptop"),
    };
    DisplayProduct(&MyProduct);
}
