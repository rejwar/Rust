use std::process::CommandArgs;

#[derive(Debug)]

enum  Command {
    Start,
    Stop,
    Pause,
    Resume,
    Shutdown,
    Unknown(String),
}

fn dispatch(cmd: Command) {
    match cmd {
        Command::Start => println!("System starting "),
        Command::Pause => println!("System is pused "),
        Command::Resume => println!("System is Resumed "),
        Command::Shutdown => println!("System is shutDown"),
        Command::Stop => println!("System is stopped"),
       _=> println!("This is unknown"),
    }
}


fn main() {
    let Command = vec![
        Command::Start,
        Command::Pause,
        Command::Resume,
        Command::Shutdown,
        Command::Stop,
        Command::Unknown(String::from("Reboot")),
    ];
    for cmd in Command {
       dispatch(cmd);
    }
}