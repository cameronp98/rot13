//! # rot13

use std::io;
use std::io::prelude::*;

/// rot13 mode
#[derive(Copy, Clone)]
pub enum Mode {
    /// shift each letter 13 places to the right (wrapping)
    Encrypt,
    /// shift each letter 13 places to the left (wrapping)
    Decrypt,
}

/// rot13 on a single byte
///
/// ```
/// use rot13::{rot13_byte, Mode};
///
/// let input = b'x';
///
/// let encrypted = rot13_byte(Mode::Encrypt, input);
/// let decrypted = rot13_byte(Mode::Decrypt, encrypted);
///
/// assert_eq!(&input, &decrypted);
/// ```
pub fn rot13_byte(mode: Mode, byte: u8) -> u8 {
    // preserve case
    let a = if byte.is_ascii_uppercase() {
        b'A'
    } else {
        b'a'
    };

    // map 'a'..'z' to 0..26
    let alphabet_pos = byte - a;

    // shift by 13 and wrap around 0..26
    let shifted_pos = match mode {
        Mode::Encrypt => (alphabet_pos + 13) % 26,
        Mode::Decrypt => {
            if alphabet_pos < 13 {
                26 - (13 - alphabet_pos)
            } else {
                alphabet_pos - 13
            }
        }
    };

    // map 0..26 back to a..z
    a + shifted_pos
}

/// rot13_slice
///
/// ```
/// use rot13::{rot13_slice, Mode};
///
/// let input = b"Hello, World!";
///
/// let encrypted = rot13_slice(Mode::Encrypt, input);
/// let decrypted = rot13_slice(Mode::Decrypt, &encrypted);
///
/// assert_eq!(input, decrypted.as_slice());
/// ```
pub fn rot13_slice(mode: Mode, input: &[u8]) -> Vec<u8> {
    input.iter()
        .map(|&byte| {
            // only apply rot13 to ascii alphabetic characters
            if byte.is_ascii_alphabetic() {
                rot13_byte(mode, byte)
            } else {
                byte
            }
        })
        .collect()
}

/// rot13 from a reader directly into a writer
///
/// continuously read bytes into a buffer, apply rot13 and write the resulting bytes
pub fn rot13<R: Read, W: Write>(mode: Mode, input: &mut R, output: &mut W) -> io::Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let amount = input.read(&mut buffer)?;

        if amount == 0 {
            break;
        }

        output.write_all(rot13_slice(mode, &buffer[..amount]).as_ref())?;
    }

    Ok(())
}
