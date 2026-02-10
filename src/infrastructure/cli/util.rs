use regex::Regex;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

pub async fn read_string_option(name: &str) -> Option<String> {
    let mut reader = BufReader::new(stdin());
    loop {
        println!("--- type {name} () ---");
        println!("(Leave empty and press Enter to set as None)");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        if value.is_empty() {
            return None
        }

        if has_whitespace(value) {
            println!("invalid input");
        } else {
            return Some(value.to_string());
        }
    }
}

pub async fn read_string_option_allow_whitespace(name: &str) -> Option<String> {
    let mut reader = BufReader::new(stdin());
    loop {
        println!("--- type {name} () ---");
        println!("(Leave empty and press Enter to set as None)");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        if value.is_empty() {
            return None
        }
        
        return Some(value.to_string());
    }
}


pub async fn read_string(name: &str) -> String {
    let mut reader = BufReader::new(stdin());
    loop {
        println!("--- type {name} ---");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        if has_whitespace(value) {
            println!("invalid input");
        } else {
            return value.to_string();
        }
    }
}

pub async fn read_int(name: &str) -> i32 {
    let mut reader = BufReader::new(stdin());
    loop {
        println!("--- type {name} ---");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        match value.parse() {
            Ok(value) => {
              return value
            },
            Err(_) => {
                println!("invalid input");
                continue;
            }
        };
    }
}

fn is_valid_input(text: &str) -> bool {

    let re = Regex::new(r"^[a-zA-Z0-9-]+$").unwrap();
    re.is_match(text)
}

fn has_whitespace(text: &str) -> bool {
text.chars().any(|c| c.is_whitespace())
}