#![forbid(unsafe_code)]

use urls::ip_addresses;

mod urls;
mod user_input;

fn main() {
    ip_addresses();
}