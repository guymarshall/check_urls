use reqwest::StatusCode;

pub fn check_urls(urls: Vec<String>) {
    for url in urls {
        match reqwest::blocking::get(&url) {
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