// Question: How to initialize non-const types globally?

use once_cell::sync::Lazy;
use std::collections::HashMap;

static CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("host".into(), "localhost".into());
    map
});

fn main() {
    println!("{:?}", CONFIG.get("host")); // Some("localhost")
}
