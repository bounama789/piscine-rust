use std::collections::HashMap;
pub use chrono::NaiveDate;

pub struct CommitData {}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week = HashMap::new();
    
    for commit in data.members() {
        let date_str = commit["commit"]["author"]["date"].as_str().unwrap();
        println!("{}",date_str);
        let date = NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%dT%H:%M:%S%Z").unwrap();
        let key = date.format("%Y-W%V").to_string();
        
        let count = commits_per_week.entry(key).or_insert(0);
        *count += 1;
    }
    
    commits_per_week
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author = HashMap::new();
    for commit in data.members() {
        let author = commit["commit"]["author"]["name"].as_str().unwrap();
        let count = commits_per_author.entry(author.to_string()).or_insert(0);
        *count += 1;
    }
    commits_per_author
}
