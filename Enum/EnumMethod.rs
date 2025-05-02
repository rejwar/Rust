enum Light {
    On,
    Off,
}



impl Light {

    fn Status (&self) -> &str {
        match self {
            Light::On => "The light is ON",
            Light::Off => " The light is off",
            
        }
    }
    
}

fn main() {
    let Bulb: Light = Light::On;
    println!("{}" , Bulb .Status());
}
