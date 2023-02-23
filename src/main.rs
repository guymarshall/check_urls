#![forbid(unsafe_code)]

use urls::ip_addresses;
use user_input::input;

mod urls;
mod user_input;

fn main() {
    let max: i32 = input("Enter max for IP check:");
    ip_addresses(max);
}