trait Notification {
    fn recipient(&self) -> String;

    fn send(&self) {
        println!("Status: Notification sent using the Generics Method");
    }
}

struct SMS {
    phone_number: String,
    message: String,
}

impl Notification for SMS {
    fn recipient(&self) -> String {
        format!("Sms to {}", self.phone_number)
    }

    fn send(&self) {
        println!(
            "ACTION: Sending sms to {} with message {}",
            self.phone_number, self.message
        );
    }
}

struct Email {
    email_address: String,
    subject: String,
}

impl Notification for Email {
    fn recipient(&self) -> String {
        format!("Email : {} ", self.email_address)
    }
}

fn main() {
    let sms_notication = SMS {
        phone_number: String::from("+8801324004824"),
        message: String::from("YOur account has been credited"),
    };

    let email_notofication = Email {
        email_address: String::from("user@gamil.com"),
        subject: String::from("Welcome"),
    };

    println!("\n -------------- Email notification is okay -------------");
    println!(" Recepient is okay --------");

    sms_notication.send();

    println!("\n ------------ Email notification (Default trait boud)");
    println!("Recepient {} ", email_notofication.recipient());
    email_notofication.send();
}
