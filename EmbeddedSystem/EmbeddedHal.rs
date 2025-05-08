use embedded_hal::digital::v2::OutputPin;

struct LED;

impl OutputPin for LED {
    fn set_high(&mut self) {
        println!("LED ON");
    }

    fn set_low(&mut self) {
        println!("LED OFF");
    }
}

fn main() {
    let mut LedDevice = LED;
    LedDevice.set_high();
    LedDevice.set_low();
}
