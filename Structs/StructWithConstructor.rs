struct  Product {
    Name: String,
    Price: f64,
}

impl Product {
    fn New(Name: &str , Price: f64 ) -> Product {
        Product { Name: Name.to_string(), Price,
         }
    }
}

fn main() {
    let Laptop: Product =  Product::New("Laptop",  1200.50);
    println!("Product {} , Price $ {}" , Laptop.Name , Laptop.Price);
}
