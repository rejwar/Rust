mod outer {
    pub mod inner {
        pub fn Run() {
            println!("Running inner module...");
        }
    }
}

use outer::inner::Run;
use inner::Run; // ❌ Error: Which "Run()"?

fn main() {
    Run(); // ❌ Ambiguous, compiler confused!
}
