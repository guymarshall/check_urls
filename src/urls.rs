use reqwest::StatusCode;
use std::fs::OpenOptions;
use std::io::Write;

pub fn check_urls(urls: Vec<String>) {
    let mut success_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("success.txt")
        .unwrap();
    let mut failure_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("failure.txt")
        .unwrap();

    for url in urls {
        match reqwest::blocking::get(&url) {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    if let Err(e) = writeln!(success_file, "{}", url) {
                        eprintln!("Error writing to success file: {}", e);
                    }
                } else {
                    if let Err(e) = writeln!(failure_file, "{} - {}", url, response.status()) {
                        eprintln!("Error writing to failure file: {}", e);
                    }
                }
            },
            Err(error) => {
                if let Err(e) = writeln!(failure_file, "{} - {}", url, error) {
                    eprintln!("Error writing to failure file: {}", e);
                }
            }
        }
    }
}

pub fn generate_urls(max: u32) -> Vec<String> {
    let mut urls = Vec::new();
    for i in 0..max {
        for j in 0..max {
            for k in 0..max {
                for l in 0..max {
                    let url = format!("http://{}.{}.{}.{}", i, j, k, l);
                    urls.push(url);
                    let url = format!("https://{}.{}.{}.{}", i, j, k, l);
                    urls.push(url);
                }
            }
        }
    }
    urls
}