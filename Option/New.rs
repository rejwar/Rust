fn get_number(flag: bool) -> Option<i32> {
    if flag {
        Some(42) 
            
        } else {
            None
        }
    }


    fn main() {
        let  Value = get_number(true);
        match  Value {
            Some(num) => println!("Received {}", num),
            None => println!("No value found"),
            
        }
    }
