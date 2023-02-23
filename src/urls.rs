use reqwest::StatusCode;
use std::fs::OpenOptions;
use std::io::Write;

pub fn check_url(url: String) {
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
    // Open the latest file with write access and truncate it
    let mut latest_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("latest.txt")
        .unwrap();

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

    // Write the latest URL to the latest file
    if let Err(e) = writeln!(latest_file, "{}", url) {
        eprintln!("Error writing to latest file: {}", e);
    }
}

pub fn ip_addresses(max: i32) {
    let latest_url = std::fs::read_to_string("latest.txt").unwrap_or("0.0.0.0".to_string());

    let latest_parts: Vec<&str> = latest_url.split('.').collect();
    let mut i = latest_parts[0].parse::<i32>().unwrap();
    let mut j = latest_parts[1].parse::<i32>().unwrap();
    let mut k = latest_parts[2].parse::<i32>().unwrap();
    let mut l = latest_parts[3].parse::<i32>().unwrap();

    for _ in i..max {
        for _ in j..max {
            for _ in k..max {
                for _ in l..max {
                    let url = format!("http://{}.{}.{}.{}", i, j, k, l);
                    check_url(url);
                    let url = format!("https://{}.{}.{}.{}", i, j, k, l);
                    check_url(url);

                    l += 1;
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