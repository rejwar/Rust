mod status;

fn main() {
    let CurrentState = status::GetStatus();
    match CurrentState {
        status::State::Active => println!("User is Active!"),
        status::State::Inactive => println!("User is Inactive!"),
    }
}
