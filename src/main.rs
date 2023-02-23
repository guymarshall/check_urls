#![forbid(unsafe_code)]

use urls::check_urls;

mod urls;

fn main() {
    let urls: Vec<&str> = vec!["https://google.co.uk", "https://notaewbsite.com"];
    check_urls(urls);
}