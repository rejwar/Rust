#[derive(Debug , Clone , Copy)]
pub struct Point {
    pub X: i32,
    pub Y: i32,
}

#[cfg(test)]

mod tests  {
    use super::*;

    #[test]
    fn test_copy_behaviour() {
        let P1 = Point {X: i32 , Y: i32 };
        let P2 = P1;
        assert_eq!(P1.X , P2.Y);
        assert_eq!(P1.X , P2.y);
    }

    #[test]
    fn test_Clone_behaviour() {
        let P1 = Point {X: 50 , Y: 60};
        let P2 = P1.clone();
        assert_eq!(P1 , P2);
    }
}