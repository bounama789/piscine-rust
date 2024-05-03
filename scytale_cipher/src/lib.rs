pub fn scytale_cipher(message: String, i: u32) -> String {
    let len = message.len();
    let padded_message = if len % i as usize == 0 {
        message.chars().collect::<Vec<char>>()
    } else {
        (message + &" ".repeat(i as usize - len % i as usize))
            .chars()
            .collect::<Vec<char>>()
    };
    let len = padded_message.len();
    let mut result = String::new();
    for j in 0..i {
        let mut k = j;
        while k < len as u32 {
            result.push(padded_message[k as usize]);
            k += i;
        }
    }
    result.trim().to_string()
}