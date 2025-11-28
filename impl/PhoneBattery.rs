struct SmartPhone {
    Battery: u32,
}

impl SmartPhone {
    fn CheckBattery(&self) {
        println!("Battery is at {}", self.Battery);
    }

    fn UsePhone(&mut self) {
        self.Battery -= 10;
        println!("Phone used . Battery now {}", self.Battery);
    }

    fn Recycle(self) {
        println!("Phone with {} battery recycled ", self.Battery);
    }
}

fn main() {
    let mut MySmartPhone = SmartPhone { Battery: 100 };
    MySmartPhone.CheckBattery();
    MySmartPhone.UsePhone();
    MySmartPhone.Recycle();
}
