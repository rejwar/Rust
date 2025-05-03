mod status {
    pub enum State {
        Active,
        Inactive,
    }
}

fn main() {
    let UserState: status::State = status::State::Active;
}
