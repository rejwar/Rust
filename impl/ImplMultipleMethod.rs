struct Recatangle {
    width: u32 ,
    height: u32 ,
}

impl Recatangle {
    fn Area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter  (&self) -> u32 {
         2 * (self.width * self.height)
    }
}