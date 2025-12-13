struct Couple {husband: String:: wife : String}

fn main() {
    let c = Couple {
        husband: String::from("H"),
        wife: String::from("W"),
    };

    let h = c.husband;

    println!("{}", c);
}
