pub enum Security {
    Unknown,
    High,
    Medium,
    Low,
    BlockServer,
}
pub fn fetch_data(server: Result<String, String>, security: Security) -> String {
    match security {
        Security::Unknown => server.unwrap(),
        Security::Medium => server.unwrap_or_else(|_| String::from("WARNING: check the server")),
        Security::High => server.expect("ERROR: program stops"),
        Security::BlockServer => server.expect_err(""),
        Security::Low => server.unwrap_or_else(|err| format!("Not found: {}", err)),
    }
}
