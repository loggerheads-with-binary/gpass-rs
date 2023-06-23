use crate::{InputStream, InputToken, LibError};
use std::io::{Read, Seek};

///A default implementation of the InputStream trait for any object that implements read and seek functionalities<br> 
///This can be used for generic custom objects of the user<br> 
impl<P: Read + Seek> InputStream for P {
    fn get_token(&mut self) -> Result<InputToken, LibError> {
        //!Read each byte until we reach a proper unicode character. Or we reach end of file
        //!Check upto where does it form a valid unicode character, wherever it does, return that as the token
        //!If w = 0, return EOF

        let mut buf = [0u8; 4];
        let w = self.read(&mut buf).map_err(|e| LibError::IOError(e))?;

        if w == 0 {
            return Ok(InputToken::EOF);
        }

        for i in 0..w {
            if buf[i] & 0b1000_0000 == 0b0000_0000 {
                //Create a utf-8 object using buf[..i]
                //Then seek back w-i bytes
                //Return the utf-8 object as the token char

                let s = std::str::from_utf8(&buf[..(i + 1)])
                    .map_err(|e| LibError::InvalidCharacter(e.to_string()))?;
                self.seek(std::io::SeekFrom::Current((w - i - 1) as i64 * -1))
                    .map_err(|e| LibError::IOError(e))?;
                match s.chars().next() {
                    Some(token) => {
                        return Ok(InputToken::Character(token));
                    }
                    None => {
                        return Err(LibError::InvalidCharacter(
                            "Invalid UTF-8 sequence".to_string(),
                        ))
                    }
                }
            }
        }

        return Err(LibError::InvalidCharacter(
            "Invalid UTF-8 sequence".to_string(),
        ));
    }
}

///A wrapper over a String to make it work with the InputStream trait (since the upstream ... does not allow to compile otherwise)<br>
pub struct IString(String);

impl IString {
    pub fn new(s: String) -> Self {
        Self(s.chars().rev().collect())
    }
}

impl InputStream for IString {
    fn get_token(&mut self) -> Result<InputToken, LibError> {
        if self.0.is_empty() {
            return Ok(InputToken::EOF);
        }

        let chr = self.0.pop().unwrap();
        return Ok(InputToken::Character(chr));
    }
}

use getch::Getch as _Getch;

///A wrapper over the getch crate to make it work with the InputStream trait (since the upstream ... does not allow to compile otherwise)<br>
#[repr(transparent)]
pub struct Getch(_Getch);

impl Getch {
    pub fn new() -> Self {
        Self(_Getch::new())
    }
}

impl InputStream for Getch {
    fn get_token(&mut self) -> Result<InputToken, LibError> {
        let ch = self.0.getch().map_err(|e| LibError::IOError(e))? as char;

        match ch {
            '\x08' | '\x7f' => Ok(InputToken::Backspace),
            '\n' | '\r' => Ok(InputToken::EOF),
            '\x03' => Err(LibError::UserInterrupt),
            _ => Ok(InputToken::Character(ch)),
        }
    }
}
