enum Command {
    Start,
    Stop,
    Pause, 
    Shutdown,
    Unknown(String),
}

fn main()  {
    let cmd = Command::Shutdown;
    match cmd {
        Command::Pause => println!("Pause the line"),
        Command::Shutdown => println!("Shutdown the pc"),
        Command::Start => println!("Starting the line .."),
        _=> println!("Command ignored"),
    }
}