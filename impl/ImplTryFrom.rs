use std::convert::TryFrom;

struct Even(i32);

impl TryFrom<i32> for Even {
    type Error =  &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if n%2 == 0 {
            Ok(Even(n))
        } else {
            Err("Not even")
        }
    }
}