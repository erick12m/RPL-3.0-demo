use std::io; // Import the standard I/O library
mod caesar_cipher;

fn main() {
    let mut user_input = String::new(); 
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_input = user_input.trim();

    let mut shift = caesar_cipher::CaesarCipher::new(user_input.len() as i32);
    let encrypted = shift.encrypt(&user_input);
    println!("{}", encrypted);
}
