struct Container <T>  {
    item : T,
}

impl <T> Container<T> {
    fn Get(&self) -> &T {
        &self.item
    }
}