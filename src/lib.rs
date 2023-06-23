#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]

#![doc = include_str!("../README.md")]


use std::sync::Arc; 
pub mod masks;

#[cfg(feature = "reverse")]
pub mod reverse;


// mod output_stream; 
// pub use output_stream::Output;

mod file_input;
pub use file_input::{Getch , IString};

//Allow users to use macros from env.rs
#[cfg(feature = "env")]
#[macro_use]
pub mod env;

#[cfg(feature = "colored")]
pub mod colors;

#[cfg(feature = "colored")]
use colored::Colorize;

///Token for implementation of the input stream<br> <br>
#[derive(Debug, Clone, Default, Copy)]
pub enum InputToken {
    ///A known unicode character<br>
    Character(char),
    ///Backspace/Delete character input from keyboards<br>
    Backspace,
    ///End of Input<br>
    EOF,
    ///These are for special cases where the token is to be ignored, and not appended to the password<br>
    #[default]
    IgnoreToken,
}

#[derive(Debug)]
///Library Error Class<br>
pub enum LibError {
    InvalidCharacter(String),

    ///For errors occured during pop_password, feed_password and end_password<br>
    PasswordCRUDFailure(String),
    UserInterrupt,
    IOError(std::io::Error),
    Other(String),
    UndefinedBehavior(String),
    ForeignLibrary(Box<dyn std::error::Error>),
}

impl Default for LibError {
    fn default() -> Self {
        Self::Other("Undefined".into())
    }
}

impl std::fmt::Display for LibError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ", self)
    }
}

impl std::error::Error for LibError {}

///Trait for character input streams (ex : Getch)<br><br>
///This is used to get character by character input from user and analyse it<br> <br>
pub trait InputStream {
    ///Getting input tokens from the input source, example keystrokes from a keyboard or characters from a file or bytes from a network source<br>
    fn get_token(&mut self) -> Result<InputToken, LibError>;
}

///Trait for password masks<br><br>
///Implemented by all the masks in this library(Standard, Blind, Echo, OneReverse, MimiReverse)<br><br>
pub trait Mask {
    ///A default implementation for the mask<br>
    fn default() -> Self
    where
        Self: Sized;

    ///To add a character to the password<br>
    fn feed_password(
        &self,
        password: &mut String,
        ch: char,
        o: &mut dyn std::io::Write,
    ) -> Result<(), String>;

    ///To pop the last character from the password<br>
    fn pop_password(&self, password: &mut String, o: &mut dyn std::io::Write)
        -> Result<(), String>;

    ///For any ending procedure(once the user has provided EOF)<br>
    fn end_password(&self, o: &mut dyn std::io::Write) -> Result<(), String> {
        o.write(b"\n").map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(feature = "colored")]
    ///Set a color for the mask<br>
    fn set_color(&mut self, _c: &str) -> ();
}

///Final main struct of the library <br>
///Standard implementation: GPass::default().get_password()?;<br>
///Consists of an input stream (similar to Getch or a file stream)<br>
///An output stream(Stderr/Stdout/file)<br>
///A mask(Standard/Blind/Echo/OneReverse/MimiReverse or user custom)<br>
///A prompt for the password input <br>
///A boolean to decide whether to return an error on user interrupt or just return the password collected till then<br>
///Prompt color(only if  `colored` feature is enabled)<br>
///For custom user implementations, use ex:
/// ```rust<br>
///     let gpass = GPass{
///     output_stream = Box::new(std::io::stdout),
///     ..Default::default()
/// }
/// ``` 
pub struct GPass {
    ///Input stream for the password<br>
    pub input_stream: Box<dyn InputStream>,

    ///Mask for the password<br>
    pub mask: Box<dyn Mask>,

    ///Output stream<br>
    pub output_stream: Box<dyn std::io::Write>,

    ///Should the result be returned as an error on userinterrupt, or just return the password collected till then<br>
    pub ctrl_c_abort: bool,

    ///Prompt for the password<br>
    pub prompt: String,

    #[cfg(feature = "colored")]
    ///Color for the prompt<br>
    pub prompt_color: Arc<str>,
}

impl Default for GPass {
    fn default() -> Self {
        Self {
            prompt: "Enter the Password".into(),
            input_stream: Box::new(Getch::new()),
            mask: Box::new(masks::Standard::default()),
            output_stream: Box::new(std::io::stderr()),
            ctrl_c_abort: true,

            #[cfg(feature = "colored")]
            prompt_color: Arc::from(colors::DEFAULT_PROMPT_COLOR),
        }
    }
}

impl GPass {
    pub fn new(
        prompt: Option<&str>,
        input_stream: Box<dyn InputStream>,
        mask: Box<dyn Mask>,
        output_stream: Box<dyn std::io::Write>,
        ctrl_c_abort: bool,
    ) -> Self {
        Self {
            prompt: prompt.unwrap_or("Enter the Password").to_string(),
            input_stream,
            mask,
            output_stream,
            ctrl_c_abort,

            #[cfg(feature = "colored")]
            prompt_color: Arc::from(colors::DEFAULT_PROMPT_COLOR),
        }
    }

    #[cfg(feature = "colored")]
    pub fn set_prompt_color(&mut self, c : &str) {
        self.prompt_color = Arc::from(c);
    }

    #[cfg(feature = "colored")]
    pub fn set_mask_color(&mut self, c: &str) {
        let _ = self.mask.set_color(c);
    }

    fn prompt_print(&mut self) -> Result<(), std::io::Error> {

        let aftermath = match self.prompt.chars().last() {
            Some(ch) if ch.is_ascii_whitespace() => "",
            _ => ": ",
        };

        let prompt = &self.prompt; 

        #[cfg(feature = "colored")]
        let prompt = prompt.to_string().color(self.prompt_color.as_ref());
        #[cfg(feature = "colored")]
        let aftermath = aftermath.color(self.prompt_color.as_ref());

        // self.output_stream.write(prompt.as_bytes())?;
        // self.output_stream.write(aftermath.as_bytes())?;
        
        write!(self.output_stream , "{}" , &prompt)?;
        write!(self.output_stream , "{}" , &aftermath)?;
        
        Ok(())
    }

    pub fn get_password(mut self) -> Result<String, LibError> {
        let mut password = String::with_capacity(25); //Default capacity

        self.prompt_print().map_err(|e| LibError::IOError(e))?;

        let mut ch;

        loop {
            match self.input_stream.get_token() {
                Ok(t) => ch = t,
                Err(LibError::IOError(e)) => {
                    return Err(LibError::IOError(e));
                }
                Err(LibError::UserInterrupt) => match self.ctrl_c_abort {
                    true => {
                        return Err(LibError::UserInterrupt);
                    }
                    false => {
                        break;
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            };

            match ch {
                InputToken::Character(c) => {
                    self.mask
                        .feed_password(&mut password, c, self.output_stream.as_mut())
                        .map_err(|e| LibError::PasswordCRUDFailure(e))?;
                }
                InputToken::Backspace => {
                    self.mask
                        .pop_password(&mut password, self.output_stream.as_mut())
                        .map_err(|e| LibError::PasswordCRUDFailure(e))?;
                }
                InputToken::EOF => {
                    self.mask
                        .end_password(&mut self.output_stream)
                        .map_err(|e| LibError::PasswordCRUDFailure(e))?;
                    break;
                }
                InputToken::IgnoreToken => {}
            }
        }

        Ok(password)
    }
}

///Void struct for output streams. Basically does not put the output anywhere<br>
pub struct Void;
impl std::io::Write for Void {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
pub mod tests;
