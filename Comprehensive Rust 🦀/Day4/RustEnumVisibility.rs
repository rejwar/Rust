mod status {
    pub enum State {
        Active,
        Inactive,
    }
}

fn main() {
    let s = status::State::Active;
}
