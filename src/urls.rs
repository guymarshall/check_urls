use reqwest::StatusCode;

pub fn check_urls(urls: Vec<String>) {
    for url in urls {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    println!("Success with {}", url);
                } else {
                    println!("Error with {}: {}", url, response.status());
                }
            },
            Err(error) => {
                println!("Error with {}: {}", url, error);
            }
        }
    }
}

pub fn generate_url_vector(max: i32) -> Vec<String> {
    let ip_address_str: String = format!("https://{}.{}.{}.{}", max, max, max, max);
    vec![ip_address_str]
}