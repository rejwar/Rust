enum Mood {
    Happy,
    Sad,
}

impl Mood {
    fn happy() -> Self {
        Mood::Happy
    }
}

fn main() {
    let m = Mood::happy();
}
