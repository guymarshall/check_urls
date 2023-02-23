use reqwest::StatusCode;

fn check_urls(urls: Vec<&str>) {
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