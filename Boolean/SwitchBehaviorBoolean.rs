fn main() {
    let HasNotifications: bool = true;

    match HasNotifications {
        true => println!("You Have New Notifications."),
        false => println!("No New Notifications."),
    }
}
