fn Greet(name: Option<&str>){
    match name {
        Some(n) => println!("Hello, {}! ", n),
        None => println!("Hello , Guest"),
        

        
    }
}

fn main(){
    Greet(Some("Alice"));
    Greet(None);
}
