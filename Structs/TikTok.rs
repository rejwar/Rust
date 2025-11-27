struct StopWatch {
    seconds: u64,
}

impl StopWatch {
    fn new() -> Self {
        Self { seconds: 0 }
    }

    fn tick(&mut self) {
        self.seconds += 1;
    }

    fn add(&mut self, s: u64) {
        self.seconds += s;
    }

    fn reset(&mut self) {
        self.seconds = 0;
    }

    fn elaspsed(&self) -> u64 {
        self.seconds
    }
}

fn main() {
    let mut sw = StopWatch::new();

    sw.tick();
    println!("Elasped {}", sw.elaspsed());

    sw.reset();
    println!("After reset = {}", sw.elaspsed());
}
