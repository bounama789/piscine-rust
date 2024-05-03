pub fn scytale_cipher(message: String, size: u32) -> String {
    let size = size as usize;
    if size == 0 {
        return message;
    }

    let mut rows = vec![String::new(); size];
    for (index, character) in message.chars().enumerate() {
        rows[index % size].push(character);
    }

    rows.concat()
}
