fn GetMax(a:i32 , b:i32) -> i32 
{{
    if a>b {
        a
    } else {
        b
    }
}
}

fn main() {
    println!("Max is :{}" , GetMax(8, 15));
}
