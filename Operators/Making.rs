fn main() {
    let state_code  = "Mh";
    let state = match state_code {
        "Mh" => {println!("Found match for MH"); "Maharastra "},
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "GOA",
        _=> "Unknown"
        
    };
    println!("State name is {}", state);
}
