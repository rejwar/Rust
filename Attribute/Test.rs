#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_ticket_equality() {
        let t1 = Ticket { title: "Rust".to_string() , status: "Open".to_string()};
        let t2 = Ticket { title: "Rust".to_string() , status: "Open".to_string()};
        assert_eq!(t1 , t2);
    }
}