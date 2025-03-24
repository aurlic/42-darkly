use reqwest::blocking::Client;
use scraper::Selector;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::process::exit;

fn main() {
    let url = "http://localhost:8080/.hidden/";
    let mut visited = HashSet::new();
    let client = Client::new();
    extract_recursive(url, &client, &mut visited);
}

fn extract_recursive(url: &str, client: &Client, visited: &mut HashSet<String>) {
    if visited.contains(&url.to_string()) {
        return;
    }
    visited.insert(url.to_string());
    let mut links = Vec::new();
    if let Ok(res) = client.get(url).send() {
        let res_txt = res.text().unwrap();
        links = extract_links(url, client, &res_txt);
    }
    for link in links {
        extract_recursive(&link, client, visited);
    }
}

fn get_full_url(base_url: &str, link_url: &str) -> String {
    format!("{}{}", base_url, link_url)
}

fn extract_links(url: &str, client: &Client, response: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    let doc = scraper::Html::parse_document(response);
    let selector = Selector::parse("a").unwrap();

    for el in doc.select(&selector) {
        if let Some(link) = el.value().attr("href") {
            if link == "../" {
                continue;
            } else if link == "README" {
                let full_link = format!("{url}README");
                if let Ok(res) = client.get(&full_link).send() {
                    if let Ok(res) = res.text() {
                        if res.contains("flag") {
                            let mut file = fs::File::create("README").expect("Error README");
                            file.write_all(&res.as_bytes()).expect("Fail to write");
                            println!("Path = {}", full_link);
                            exit(0);
                        }
                    }
                }
            }
            let full_url = get_full_url(url, &link);
            links.push(full_url.to_string());
        }
    }
    links
}
