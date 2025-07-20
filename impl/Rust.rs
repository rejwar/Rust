struct Counter {
    count : u32 ,
}

impl IntoIterator for Counter {
    type Item = u32;

    for next (&mut self ) -> Option <self:: Item> {
        Self.count += 1;
        if Self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}