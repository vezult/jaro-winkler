use std::io;
use std::io::BufRead;

use gumdrop::Options;

mod jaro;

#[derive(Debug, Options)]
struct CliOptions {
    #[options(free)]
    strings: Vec<String>,
}

fn split_line(line: String) -> (String, Option<String>) {
    let parts: Vec<&str>;
    let trimmed_line = line.trim();
    let head = if trimmed_line.starts_with("\"") {
        parts = trimmed_line
            .trim_start_matches('"')
            .splitn(2, '"')
            .collect();
        parts[0].trim_end_matches('"').to_string()
    } else {
        parts = trimmed_line.splitn(2, char::is_whitespace).collect();
        parts[0].trim_end_matches(char::is_whitespace).to_string()
    };

    let tail = if parts.len() > 1 {
        Some(parts[1].to_string())
    } else {
        None
    };

    (head, tail)
}

fn main() {
    let opts = CliOptions::parse_args_default_or_exit();

    let num_args = opts.strings.len();
    if num_args == 0 {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(data) = line {
                let (s1, remainder) = split_line(data);
                let (s2, _) = match remainder {
                    Some(r) => split_line(r),
                    None => ("".to_string(), None),
                };

                let jaro_score = jaro::jaro(&s1.to_string(), &s2.to_string());
                let jw_score = jaro::winkler(&s1.to_string(), &s2.to_string());
                println!("{} {} => j: {} jw: {}", s1, s2, jaro_score, jw_score);
            }
        }
    } else if num_args == 2 {
        let s1 = &opts.strings[0];
        let s2 = &opts.strings[1];
        let jaro_score = jaro::jaro(s1, s2);
        let jw_score = jaro::winkler(s1, s2);
        println!("{}, {} => j: {} jw: {}", s1, s2, jaro_score, jw_score);
    } else {
        eprintln!("Error: two strings must be provided.");
    }
}
