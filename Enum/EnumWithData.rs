enum File {
    Open { name: String, size: u32 },
    Closed,
}

fn main() {
    let file = File::Open {
        name: String::from("data.txt"),
        size: 512,
    };
    match file {
        File::Open { name, size } => println!("File Name: {}, Size: {}", name, size),
        File::Closed => println!("File is closed."),
    }
}
