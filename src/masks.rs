//!Defines basic masks for the password input<br> 
//!Standard, Blind and Echo are supported as default masks<br> 

use crate::Mask;
use std::sync::Arc;

#[cfg(feature = "colored")]
use crate::colors;
#[cfg(feature = "colored")]
use colored::Colorize;

pub const DEFAULT_MASK_CHAR: &'static str = "*";

///Standard Mask <br>
///This consists of an internal mask string, say "#", and will cover your characters with said mask, ex: Hello will be masked to #####<br>
pub struct Standard {
    #[cfg(feature = "colored")]
    mask: colored::ColoredString,

    #[cfg(not(feature = "colored"))]
    mask: Arc<str>,

    spaces: Arc<str>,
    backs: Arc<str>,
}

impl Mask for Standard {
    fn default() -> Self {
        Self::new(DEFAULT_MASK_CHAR)
    }

    fn feed_password(
        &self,
        password: &mut String,
        ch: char,
        o: &mut dyn std::io::Write,
    ) -> Result<(), String> {
        // o.write(self.mask.as_bytes()).map_err(|e| e.to_string())?;
        write!(o , "{}" , self.mask).map_err(|e| e.to_string())?;
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
                write!(o , "{}" , buffer).map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    #[cfg(feature = "colored")]
    fn set_color(&mut self, c: &str) -> () {
        self.mask = self.mask.clone().color(c);
    }
}

impl Standard {
    pub fn new(mask: &str) -> Self {
        #[cfg(feature = "colored")]
        let mask = colored::ColoredString::from(mask).color(colors::DEFAULT_MASK_COLOR);

        #[cfg(not(feature = "colored"))]
        let mask: Arc<str> = Arc::from(mask);

        Self {
            spaces: Arc::from(" ".repeat(mask.len())),
            backs: Arc::from("\x08".repeat(mask.len())),
            mask,
        }
    }
}

///Essentially this is no mask<br>
///This functionality is primarily provided for cleartext inputs <br>
///Ex: if your program may have a flag to take cleartext input and you might still want to use the library <br>
///Then this mask becomes handy <br>
pub struct Echo {
    #[cfg(feature = "colored")]
    color: Arc<str>,
}

impl Mask for Echo {
    fn default() -> Self {
        Self {
            #[cfg(feature = "colored")]
            color: Arc::from(colors::DEFAULT_MASK_COLOR),
        }
    }

    fn feed_password(
        &self,
        password: &mut String,
        ch: char,
        o: &mut dyn std::io::Write,
    ) -> Result<(), String> {
        let chs = ch.to_string();

        #[cfg(feature = "colored")]
        let chs = chs.color(self.color.as_ref());

        // o.write(chs.as_bytes()).map_err(|e| e.to_string())?;
        
        write!(o, "{}", chs).map_err(|e| e.to_string())?;
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
                o.write(b"\x08 \x08").map_err(|e| e.to_string())?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    #[cfg(feature = "colored")]
    fn set_color(&mut self, c: &str) -> () {
        self.color = Arc::from(c);
        // Ok(())
    }
}

#[derive(Default)]
///Essentially the equivalent of bash read <br>
///No characters are echoed to the console/output <br>
///This is similar to setting the lib::Void as the output stream<br>
pub struct Blind;

impl Mask for Blind {
    fn default() -> Self {
        Default::default()
    }

    fn end_password(&self, _o: &mut dyn std::io::Write) -> Result<(), String> {
        Ok(())
    }

    fn feed_password(
        &self,
        password: &mut String,
        ch: char,
        _o: &mut dyn std::io::Write,
    ) -> Result<(), String> {
        password.push(ch);
        Ok(())
    }

    fn pop_password(
        &self,
        password: &mut String,
        _o: &mut dyn std::io::Write,
    ) -> Result<(), String> {
        let _ = password.pop();
        Ok(())
    }

    #[cfg(feature = "colored")]
    fn set_color(&mut self, _color: &str){}
}
