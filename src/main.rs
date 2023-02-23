#![forbid(unsafe_code)]

fn main() {
    let urls: Vec<&str> = vec!["https://google.co.uk", "https://notaewbsite.com"];
    check_urls(urls);
}