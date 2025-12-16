trait Storage {
    fn save(&self, data: &str);
    fn load(&self) -> String;
}

struct MemoryStorage {
    buffer: String,
}

struct FileStorage {
    filename: String,
}

impl Storage for MemoryStorage {
    fn save(&self, data: &str) {
        println!("Saving to  memory {}", data);
    }

    fn load(&self) -> String {
        "Memory data".to_string()
    }
}

impl Storage for FileStorage {
    fn save(&self, data: &str) {
        println!("Saving to file {} {}", self.filename, data);
    }

    fn load(&self) -> String {
        "File data".to_string()
    }
}

fn use_storage(store: &dyn Storage) {
    store.save("Hello");
    println!("Loaded {}", store.load());
}

fn main() {
    let mem = MemoryStorage {
        buffer: String::new(),
    };
    let file = FileStorage {
        filename: "data.txt ".to_string(),
    };

    use_storage(&mem);
    use_storage(&file);
}
