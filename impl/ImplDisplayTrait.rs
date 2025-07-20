use std::fmt;

struct User {
    name: String,
}

impl fmt::Display for User  {
    fn fmt (&self , f: &mut fmt::Formatter) -> fmt::Result {
        write!(f , "User  {}", self.name )
    }
}