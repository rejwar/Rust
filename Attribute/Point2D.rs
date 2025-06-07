/// Represents a point in 2D space.
///
/// # Fields
///
/// - `X`: The x-coordinate of the point.
/// - `Y`: The y-coordinate of the point.
#[derive(Debug)]
pub struct Point {
    pub X: i32,
    pub Y: i32,
}

fn main() {
    // Example usage of Point
    let p = Point::New(10, 20);
    println!("{:?}", p);
}

impl Point {
    /// Creates a new point with the given coordinates.
    ///
    /// # Arguments
    ///
    /// - `x`: The x-coordinate.
    /// - `y`: The y-coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// let p = Point::new(10, 20);
    /// assert_eq!(p.X, 10);
    /// assert_eq!(p.Y, 20);
    /// ```
    pub fn New(X: i32, Y: i32) -> Point {
        Point { X, Y }
    }
}
impl Default for Point {
    /// Creates a default point at the origin (0, 0).
    ///
    /// # Example
    ///
    /// ```
    /// let p: Point = Default::default();
    /// assert_eq!(p.X, 0);
    /// assert_eq!(p.Y, 0);
    /// ```
    fn default() -> Self {
        Point { X: 0, Y: 0 }
    }
}
