#![forbid(unsafe_code)]

use reqwest::StatusCode;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::Duration;
use reqwest::blocking::Client;

fn create_success_file() -> File {
    let success_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("success.txt")
        .unwrap();
    success_file
}

fn create_client() -> Client {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    client
}

fn check_url(url: String, success_file: &mut File, client: &Client) {
    match client.get(url.clone()).send() {
        Ok(response) => {
            println!("Success: {}", &url);
            if response.status() == StatusCode::OK {
                if let Err(e) = writeln!(success_file, "{}", url) {
                    eprintln!("Error writing to success file: {}", e);
                }
            }
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

pub fn ip_addresses() {
    let mut success_file = create_success_file();
    let client = create_client();

    const MAX: i32 = 255;
    let mut latest_url: String = String::new();
    if let Ok(contents) = std::fs::read_to_string("success.txt") {
        latest_url = contents.trim().to_string();
    }
    let latest_url: &str = latest_url.trim_start_matches("http://").trim_start_matches("https://");
    let latest_parts: Vec<&str> = latest_url.split('.').collect();

    let length: i32 = latest_parts.len().try_into().unwrap();

    let mut i: i32 = if length == 4 {latest_parts[0].parse::<i32>().unwrap_or(0)} else {0};
    let mut j: i32 = if length == 4 {latest_parts[1].parse::<i32>().unwrap_or(0)} else {0};
    let mut k: i32 = if length == 4 {latest_parts[2].parse::<i32>().unwrap_or(0)} else {0};
    let mut l: i32 = if length == 4 {latest_parts[3].parse::<i32>().unwrap_or(0)} else {0};

    for _ in i..MAX {
        for _ in j..MAX {
            for _ in k..MAX {
                for _ in l..MAX {
                    if i <= 255 && j <= 255 && k <= 255 && l <= 255 {
                        let url: String = format!("http://{}.{}.{}.{}", i, j, k, l);
                        check_url(url, &mut success_file, &client);
                        let url: String = format!("https://{}.{}.{}.{}", i, j, k, l);
                        check_url(url, &mut success_file, &client);

                        l += 1;
                    } else if i > 255 {
                        return;
                    }
                }
                l = 0;
                k += 1;
            }
            k = 0;
            j += 1;
        }
        j = 0;
        i += 1;
    }
}