mod Greetings;

use Greetings::English::Hello as EnglishHello;
use Greetings::Bangla::Hello as BanglaHello;

fn main() {
    EnglishHello();
    BanglaHello();
}
