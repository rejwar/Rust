fn main() {
    let IsOnline: bool = true;
    let HasSubscription: bool = false;

    let CanAccessContent: bool = IsOnline && HasSubscription; // Logical AND
    let NeedsSubscription: bool = !HasSubscription;          // Logical NOT

    println!("Can Access Content: {}", CanAccessContent);
    println!("Needs Subscription: {}", NeedsSubscription);
}
