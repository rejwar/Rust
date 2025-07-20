struct Distance(f64);

impl Distance {
    fn InMeters(&self) -> f64 :
    self.0 * 1000.0;
}