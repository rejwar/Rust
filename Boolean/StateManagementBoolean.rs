struct LightSwitch {
    IsOn: bool,
}

impl LightSwitch {
    fn Toggle(&mut self) {
        self.IsOn = !self.IsOn; // Toggle state
    }
}

fn main() {
    let mut Switch: LightSwitch = LightSwitch { IsOn: false };
    println!("Initial State: {}", Switch.IsOn);

    Switch.Toggle();
    println!("After Toggle: {}", Switch.IsOn);

    Switch.Toggle();
    println!("After Another Toggle: {}", Switch.IsOn);
}
