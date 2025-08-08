// resolution, should be hidden for students

pub struct CaesarCipher {
    shift: i32,
}

impl CaesarCipher {
    pub fn new(shift: i32) -> Self {
        CaesarCipher {
            shift: shift,
        }
    }

    pub fn encrypt(&self, message: &str) -> String {
        message.chars().map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { 'A' as u8 } else { 'a' as u8 };
                let normalized_shift = ((self.shift % 26) + 26) % 26; // Handle negative shifts
                let shifted = (c as u8 - base + normalized_shift as u8) % 26;
                (shifted + base) as char
            } else {
                c
            }
        }).collect()
    }
}