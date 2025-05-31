fn main() {
    let x:i32 = 5;
    assert_eq!(" u32".to_string(), format!(" {}", x));
    assert_eq!(" 5".to_string(), format!(" {}", x));
    assert_eq!(" 5".to_string(), format!(" {}", x));
    assert_eq!(" 5".to_string(), format!(" {}", x));
    assert_eq!(" 5".to_string(), format!(" {}", x));
    println!("All assertions passed!");
}
