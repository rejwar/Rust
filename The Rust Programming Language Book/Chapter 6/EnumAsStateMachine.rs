// 
enum State {
    Start,
    Processing,
    Done,
}

impl State {
    fn next(self) -> Self {
        match self {
            State::Start => State::Processing,
            State::Processing => State::Done,
            State::Done => State::Done,
        }
    }
}

fn main() {
    let mut state = State::Start;
    for _ in 0..3 {
        println!("Current state: {:?}", std::mem::discriminant(&state));
        state = state.next();
    }
}
