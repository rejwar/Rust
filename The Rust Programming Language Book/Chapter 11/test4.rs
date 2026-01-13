#[test]

fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("2 or 2 are not making 4 "))
    }
}
