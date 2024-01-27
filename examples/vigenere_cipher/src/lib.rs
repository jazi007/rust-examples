//! [Vigenère cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher): encoder / decoder
//!
//! A simple implementation that accepts only alphabetics and spaces
//!
//! Also non case sensitive all characters are considered as Upper Case
//!

/// Cipher type
pub struct Cipher<'a> {
    key: &'a str,
}

impl<'a> Cipher<'a> {
    const ALPHABET: &'static [u8; 26] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    /// Construct a new [`Cipher`] with a new key
    ///
    /// # Panic
    /// This function panics if the key is empty
    ///
    /// # Example
    /// ```rust
    /// use cipher::Cipher;
    /// let text = "Hello world";
    /// let key = "AbCd";
    /// let cipher = Cipher::new(key);
    /// let encoded = cipher.encrypt(text).unwrap();
    /// let decoded = cipher.decrypt(&encoded).unwrap();
    /// assert!(decoded.eq(&text.to_uppercase()));
    /// ```
    pub const fn new(key: &'a str) -> Self {
        assert!(!key.is_empty());
        Self { key }
    }
    /// Decrypt a value using the provided key
    ///
    /// # Example
    /// ```rust
    /// use cipher::Cipher;
    /// let invalid_text = "AAA 12345";
    /// let key = "AbCd";
    /// let cipher = Cipher::new(key);
    /// let decoded = cipher.decrypt(invalid_text);
    /// assert!(matches!(decoded, Err('1')));
    /// ```
    pub fn decrypt(&self, value: &str) -> Result<String, char> {
        let mut key = self.key.chars().cycle();
        Ok(value
            .split(' ')
            .map(|word| {
                word.chars()
                    .zip(key.by_ref())
                    .map(|(c, k)| (c.to_ascii_uppercase(), k.to_ascii_uppercase()))
                    .map(|(c, k)| match c {
                        'A'..='Z' => {
                            let oi: u32 = (u32::from(c) + 26 - u32::from(k)).rem_euclid(26);
                            Ok(char::from_u32(Self::ALPHABET[oi as usize].into()).unwrap())
                        }
                        _ => Err(c),
                    })
                    .collect()
            })
            .collect::<Result<Vec<String>, char>>()?
            .join(" "))
    }
    /// Encrypt the value using the provided Key
    ///
    /// # Example
    /// ```rust
    /// use cipher::Cipher;
    /// let text = "AWESOME RUST";
    /// let key = "RUST";
    /// let cipher = Cipher::new(key);
    /// let encoded = cipher.encrypt(text);
    /// assert!(encoded.is_ok());
    /// println!("{:?}", encoded);
    /// ```
    pub fn encrypt(&self, value: &str) -> Result<String, char> {
        let mut key = self.key.chars().cycle();
        Ok(value
            .split(' ')
            .map(|word| {
                word.chars()
                    .zip(key.by_ref())
                    .map(|(c, k)| (c.to_ascii_uppercase(), k.to_ascii_uppercase()))
                    .map(|(c, k)| match c {
                        'A'..='Z' => {
                            let oi: u32 = (u32::from(c) + u32::from(k)).rem_euclid(26);
                            Ok(char::from_u32(Self::ALPHABET[oi as usize].into()).unwrap())
                        }
                        _ => Err(c),
                    })
                    .collect()
            })
            .collect::<Result<Vec<String>, char>>()?
            .join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn empty_key() {
        let _ = Cipher::new("");
    }
}
