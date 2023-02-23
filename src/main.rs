#![forbid(unsafe_code)]

use urls::check_urls;
use user_input::input;

mod urls;
mod user_input;

fn main() {
    let max: i32 = input("Enter max for IP check:");
    let urls: Vec<&str> = vec!["https://google.co.uk", "https://notaewbsite.com"];
    check_urls(urls);
}