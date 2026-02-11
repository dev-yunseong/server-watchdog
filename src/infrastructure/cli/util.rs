use std::sync::OnceLock;
use regex::Regex;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

pub async fn read_string_option(name: &str, format_checker: FormatChecker) -> Option<String> {
    let mut reader = BufReader::new(stdin());
    loop {
        println!("--- type {name} ---");
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

        if format_checker.valid(value) {
            return Some(value.to_string());
        } else {
            println!("invalid input");
        }
    }
}

pub async fn read_string(name: &str, format_checker: FormatChecker) -> String {
    let mut reader = BufReader::new(stdin());
    loop {
        println!("--- type {name} ---");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        if format_checker.valid(value) {
            return value.to_string();
        } else {
            println!("invalid input");
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

pub enum FormatChecker {
    BaseUrl,
    Name,
    NotAllowWhitespace,
    None,
}

impl FormatChecker {
    pub fn valid(&self, text: &str) -> bool {
        match self {
            FormatChecker::BaseUrl => {
                static RE: OnceLock<Regex> = OnceLock::new();
                RE.get_or_init(||{Regex::new(r"^[a-zA-Z]+://[a-zA-Z0-9]+(:[0-9]+)?$").unwrap()})
                    .is_match(text)
            },
            FormatChecker::Name => {
                static RE: OnceLock<Regex> = OnceLock::new();
                RE.get_or_init(||{Regex::new(r"^[a-zA-Z0-9-]+$").unwrap()})
                    .is_match(text)
            },
            FormatChecker::NotAllowWhitespace => {
                !text.chars().any(|c| c.is_whitespace())
            },
            FormatChecker::None => true
        }
    }
}