#[derive(Debug)]
pub struct  Ticket {
    pub Title: String,
    pub Status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn TestDebug() {
        let T = Ticket {Title: "Rust".to_string(), Status: "Open".to_string()};
        assert_eq!(T.Status, "Open");
    }
}