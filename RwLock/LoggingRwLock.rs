use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let LogEntries = Arc::new(RwLock::new(vec!["Start".to_string()]));
    
    let Writer = {
        let LogRef = Arc::clone(&LogEntries);
        thread::spawn(move || {
            let mut Log = LogRef.write().unwrap();
            Log.push("Processing Data".to_string());
        })
    };

    let Reader = {
        let LogRef = Arc::clone(&LogEntries);
        thread::spawn(move || {
            let Log = LogRef.read().unwrap();
            println!("Log Entries: {:?}", *Log);
        })
    };

    Writer.join().unwrap();
    Reader.join().unwrap();
}
