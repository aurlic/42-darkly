use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;

fn main() {
    let base_url = "http://localhost:8080/.hidden/";
    let client = Client::new();
    let mut visited = HashSet::new();

    if let Some(flag_path) = scrape(&client, base_url, &mut visited) {
        println!("[+] Flag found in: {}", flag_path);
    } else {
        println!("[-] No flag found.");
    }
}

fn scrape(client: &Client, url: &str, visited: &mut HashSet<String>) -> Option<String> {
    if !visited.insert(url.to_string()) {
        return None;
    }

    let response = client.get(url).send().ok()?;
    let body = response.text().ok()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href == "../" {
                continue;
            }
            let new_url = format!("{}{}", url, href);

            if href == "README" {
                if let Some(flag) = check_readme_content(client, &new_url) {
                    return Some(flag);
                }
            } else {
                if let Some(flag) = scrape(client, &new_url, visited) {
                    return Some(flag);
                }
            }
        }
    }
    None
}

fn check_readme_content(client: &Client, url: &str) -> Option<String> {
    let response = client.get(url).send().ok()?;
    let content = response.text().ok()?;

    if content.contains("flag") {
        return Some(url.to_string());
    }
    None
}
