#[cfg(target_os = "windows")] // ✅ শুধুমাত্র Windows OS-এ কোড রান হবে
fn PlatformSpecific() {
    println!("Running on Windows!");
}

#[cfg(target_os = "linux")] // ✅ শুধুমাত্র Linux OS-এ কোড রান হবে
fn PlatformSpecific() {
    println!("Running on Linux!");
}

fn main() {
    PlatformSpecific();
}
