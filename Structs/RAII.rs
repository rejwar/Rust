struct SecureData {
    secret: String,
}

impl SecureData {
    fn New(secret: String) -> Self {
        Self { secret }
    }

    fn Show(&self) {
        println!("Secret length {}", self.secret.len());
    }
}

impl Drop for SecureData {
    fn drop(&mut self) {
        println!(" Dropping Secure Data.. wipping secret ");
        self.secret.clear();
    }
}

fn main() {
    {
        let data = SecureData::New(String::from("TOP_SECRET_KEY"));
        data.Show();
    }

    println!(" End of Main");
}
