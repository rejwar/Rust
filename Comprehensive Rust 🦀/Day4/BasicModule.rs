mod Greetings { 
    pub fn Hello() {
        println!( "Hello from the greetings module "  );
    }
}

use Greetings::Hello;

fn main() {
    Hello();
}
