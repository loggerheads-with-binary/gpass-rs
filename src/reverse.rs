//!Defines the one reverse and mimi reverse masks for password input <br><br>
//! OneReverse shows the last character in clear-display (similar to JS prompts on websites)<br>
//! MimiReverse shows the last character in clear-display even when backslashes are pressed<br>
//!The masks are only available if the `reverse` feature flag is enabled<br>

use crate::masks::DEFAULT_MASK_CHAR;
use crate::Mask;
use std::sync::Arc;

#[cfg(feature = "colored")]
use crate::colors;

#[cfg(feature = "colored")]
use colored::{Color, Colorize};

///Mask that shows last character of password in plaintext and rest all characters masked<br>
///Similar to JS prompts on websites<br>
pub struct OneReverse {
    mask: Arc<str>,
    spaces: Arc<str>,
    backs: Arc<str>,

    #[cfg(feature = "colored")]
    color: Arc<str>,
}

impl OneReverse {
    pub fn new(mask: &str) -> Self {
        Self {
            mask: Arc::from(mask),
            spaces: Arc::from(" ".repeat(mask.len())),
            backs: Arc::from("\x08".repeat(mask.len())),
        
            #[cfg(feature = "colored")]
            color: Arc::from(colors::DEFAULT_MASK_COLOR),
        }
    }
}


impl Mask for OneReverse {
    fn default() -> Self {
        Self::new(DEFAULT_MASK_CHAR)
    }

    fn feed_password(
        &self,
        password: &mut String,
        ch: char,
        o: &mut dyn std::io::Write,
    ) -> Result<(), String> {
        let buffer = match password.is_empty() {
            true => ch.to_string(),
            false => {
                format!("\x08{mask}{ch}", mask = &self.mask)
            }
        };

        #[cfg(feature = "colored")]
        let buffer = buffer.color(self.color.as_ref());

        write!(o , "{}", buffer).map_err(|e| e.to_string())?;
        password.push(ch);
        Ok(())
    }

    fn pop_password(
        &self,
        password: &mut String,
        o: &mut dyn std::io::Write,
    ) -> Result<(), String> {
        match password.pop() {
            Some(_) => {
                let buffer = format!(
                    "{backs}{spaces}{backs}",
                    backs = &self.backs,
                    spaces = &self.spaces
                );

                #[cfg(feature = "colored")]
                let buffer = buffer.color(self.color.as_ref());

                write!(o , "{}", buffer).map_err(|e| e.to_string())?;
            }
            None => {}
        };

        Ok(())
    }

    fn end_password(&self, o: &mut dyn std::io::Write) -> Result<(), String> {
        let buffer = format!("\x08{mask}\n", mask = &self.mask);

        #[cfg(feature = "colored")]
        let buffer = buffer.color(self.color.as_ref());

        // o.write(buffer.as_bytes()).map_err(|e| e.to_string())?;
        write!(o , "{}" , buffer).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(feature = "colored")]
    fn set_color(&mut self, c: &str) -> () {
        self.color = Arc::from(c);
    }
}

///Composition due to lack of inheritance in Rust<br>
///Added functionality with OneReverse that the last character becomes visible during backspaces. <br>
///So ex: if you enter Hello, then it might look like *****o, but once you press backspace, it will look like ***l<br>
///This functionality is additional wrt OneReverse <br>
///Rest everything is same <br>
pub struct MimiReverse(OneReverse);

impl MimiReverse {
    pub fn new(mask: &str) -> Self {
        Self(OneReverse::new(mask))
    }
}

impl Mask for MimiReverse {
    fn default() -> Self {
        Self::new(DEFAULT_MASK_CHAR)
    }

    fn feed_password(
        &self,
        password: &mut String,
        ch: char,
        o: &mut dyn std::io::Write,
    ) -> Result<(), String> {
        self.0.feed_password(password, ch, o)
    }

    fn pop_password(
        &self,
        password: &mut String,
        o: &mut dyn std::io::Write,
    ) -> Result<(), String> {

        let buffer = match password.len() {
            0 => {
                return Ok(());
            }
            1 => "\x08 \x08".to_string(),
            _ => {
                format!(
                    "\x08{backs} {spaces}{backs}\x08{last_char}",
                    backs = &self.0.backs,
                    spaces = &self.0.spaces,
                    last_char = &password.chars().rev().take(2).last().unwrap()
                )
            }
        };

        #[cfg(feature = "colored")]
        let buffer = buffer.color(self.0.color.as_ref());

        let _ = password.pop();

        // o.write(buffer.as_bytes()).map_err(|e| e.to_string())?;
        write!(o , "{}", buffer).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn end_password(&self, o: &mut dyn std::io::Write) -> Result<(), String> {
        self.0.end_password(o)
    }

    #[cfg(feature = "colored")]
    fn set_color(&mut self, c: &str) -> () {
        self.0.set_color(c);
    }
}
