use std::collections::BTreeSet;

fn main() {
    let mut ActiveConnections = BTreeSet::new();
    ActiveConnections.insert("192.168.1.10");
    ActiveConnections.insert("192.168.1.20");

    println!("Active Connections: {:?}", ActiveConnections);
}
