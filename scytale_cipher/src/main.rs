use scytale_cipher::scytale_cipher;

fn main() {
    let message = "Hello, World!".to_string();
    let size = 12;
    let encrypted = scytale_cipher(message, size);

    println!(
        "\"scytale Code\" size=6 -> {:?}",
        scytale_cipher(String::from("scytale Code"), 6)
    );
    println!(
        "\"scytale Code\" size=8 -> {:?}",
        scytale_cipher(String::from("scytale Code"), 8)
    );
    println!("Encrypted message: {}", encrypted);
}
