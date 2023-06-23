//!A set of macros to access prompt and mask values from the environment variables <br>
//!This is to be used with the optional feature flag `env`<br>
//!Provided macros -> env_prompt, env_mask, mask_env_color, prompt_env_color <br>
//!Note, even though env_color! has been exported, it is not supposed to be used by itself. Use the mask_env_color and prompt_env_color macros instead<br>

use crate::Mask;
use regex::Regex;
use std::sync::RwLock;

///Get prompt from the environment if available, else default to default_prompt<br>
///Inner function for the macro env_prompt!<br>
pub fn prompt_from_env(env_var: &str, default_prompt: &str) -> String {
    match std::env::var(env_var) {
        Ok(prompt) => prompt,
        Err(_) => default_prompt.to_string(),
    }
}

#[macro_export]
///Used to get the password prompt eg. "Enter Password Here: " from the environment variables <br>
///By default the `GPASS_PROMPT` environment variable is evaluated, and if it is not set, it returns default "Enter Password Here: " <br>

///This macro can be used 4 ways <br>
/// env_prompt!() -> Uses `GPASS_PROMPT` env key and defaults to "Enter Password Here: "<br>
/// env_prompt!("MY_ENV_VAR") : Customize the environment variable <br>
/// env_prompt!(default = "My Prompt") : Customize the prompt <br>
/// env_prompt!("MY_ENV_VAR" , "My Prompt" ) : Customize both <br>
macro_rules! env_prompt {
    


    () => {
        prompt_from_env("GPASS_PROMPT", "Enter Password Here: ")
    };

    ($prompt : expr , $default : expr) => {
        prompt_from_env($prompt, $default)
    };

    ($prompt : expr ) => {
        prompt_from_env($prompt, "Enter Password Here: ")
    };

    //Add identifier so we can use prompt_from_env!(default = "default")
    (default = $default : expr) => {
        prompt_from_env("GPASS_PROMPT", $default)
    };
}

///Get mask from the environment if available<br>
///Inner function for the macro env_mask!<br>
pub fn mask_from_env(
    env_var: &str,
    converters: &[fn(&str) -> Option<Box<dyn Mask>>],
) -> Option<Box<dyn Mask>> {
    let mask = std::env::var(env_var);
    match mask{
        Ok(val) => mask_from_str(&val , converters),
        Err(_) => None
    }
}



pub fn mask_from_str(val: &str,      converters: &[fn(&str) -> Option<Box<dyn Mask>>]) -> Option<Box<dyn Mask>>{

    for converter in converters {
        if let Some(mask) = converter(&val) {
            return Some(mask);
        }
    }

    None
}

///Helper function for regex based mask obtains<br>
///Given a base_string(say "standard"), it matches against standard -> returns Standard::default()<br>
///It also matches against Standard(<mask_string>) and Standard[<mask_string>] and Standard{<mask_string>} to return Standard{mask : <mask_string>}<br>
/// These matches are case insensitive<br>
pub fn mask_string_get(base_string: &str, check_string: &str) -> Option<String> {
    
    let re = format!(r"(?i){base_string}");

    if Regex::new(re.as_str()).unwrap().is_match(check_string) {
        return Some(crate::masks::DEFAULT_MASK_CHAR.to_string());
    }
    
    let re1 = format!(r"(?i){base_string}\((?P<mask_string>.*)\)");
    let re2 = format!(r"(?i){base_string}\[(?P<mask_string>.*)\]");
    let re3 = format!(r"(?i){base_string}\{{(?P<mask_string>.*)\}}");

    //?i sets up case insensitive

    for regular_expression in [re1, re2, re3].iter() {
        let re = Regex::new(regular_expression.as_str()).unwrap();
        if let Some(captures) = re.captures(check_string) {
            return match captures.name("mask_string") {
                Some(ref t) => Some(t.to_owned().as_str().to_string()),
                None => None,
            };
        }
    }

    None
}

lazy_static::lazy_static! {

    #[allow(unused_parens)]

    ///Default Converter Functions<br>
    ///Each converter function uses the signature fn(&str) -> Option<Box<dyn Mask>><br>
    ///A converter is supposed to parse the string input for whatever pattern matches that mask <br>
    /// Ex: "blind" will be accepted only by the Converter for the Blind mask, none else <br>
    /// A converter shows success by returning Some(variant) and failure by returning None variant <br>
    /// You can also provide your own converters for custom masks <br>
    pub static ref DEFAULT_CONVERTERS : RwLock<Vec<fn(&str) -> Option<Box<dyn Mask>>>> = {

        let converters : Vec<fn(&str) -> Option<Box<dyn Mask>>> = vec![

            |s : &str| {
                if s.trim().to_lowercase() == "blind" {
                    Some(Box::new(crate::masks::Blind{}))
                }
                else {None}
            },
            |s : &str| {
                if s.trim().to_lowercase() == "echo" {
                    Some(Box::new(crate::masks::Echo::default()))
                }
                else {None}
            },

            |s : &str| {

                let s = s.trim();
                match mask_string_get("standard" , s){
                    Some(res) => Some(Box::new(crate::masks::Standard::new(&res))),
                    None => None
                }

                // if s == "standard"{
                //     return Some(Box::new(crate::masks::Standard::default()))
                // }

                // //The regular expression formed is such that Standard[..] or Standard(..) or Standard{..}
                // //Create a single regex that will extract the .. from inside the string

                // for re in [
                //     r"standard\(?<mask_string>.*\)",
                //     r"standard\[?<mask_string>.*\]",
                //     r"standard\{?<mask_string>.*\}",
                // ]{
                //     //Extract name from s using regex
                //     let re = Regex::new(re).unwrap();
                //     let caps = re.captures(s);
                //     if caps.is_none(){
                //         continue;
                //     }
                //     let caps = caps.unwrap();
                //     let mask_string = caps.name("mask_string");
                //     if mask_string.is_none(){
                //         continue;
                //     }
                //     let mask_string = mask_string.unwrap().as_str();
                //     return Some(Box::new(crate::masks::Standard::new(mask_string)))
                // }


                // None
            },

            #[cfg(feature = "reverse")]
            |s : &str| {

                let s = s.trim();
                match mask_string_get("onereverse" , s){
                    Some(res) => return Some(Box::new(crate::reverse::OneReverse::new(&res))),
                    None => {}
                };

                match mask_string_get("or" , s){
                    Some(res) => return Some(Box::new(crate::reverse::OneReverse::new(&res))),
                    None => None
                }
            },

            #[cfg(feature = "reverse")]
            |s : &str| {
                let s = s.trim();
                match mask_string_get("mimi" , s){
                    Some(res) => return Some(Box::new(crate::reverse::MimiReverse::new(&res))),
                    None => {}
                };

                match mask_string_get("mimireverse" ,s) {
                    Some(res) => return Some(Box::new(crate::reverse::MimiReverse::new(&res))),
                    None => {}
                };

                match mask_string_get("mr" , s){
                    Some(res) => return Some(Box::new(crate::reverse::MimiReverse::new(&res))),
                    None => None
                }
            }

        ];

        RwLock::new(converters)
    };

}

#[macro_export]
///Macro to get a mask from the environment <br>
///By default uses the environment variable `GPASS_MASK`<br>
///Examples of usage:<br>
/// 1. `env_mask!()` -> Returns from environment variable `GPASS_MASK`<br>
/// 2. `env_mask!("MY_ENV_VAR")` -> Returns from custom env variable<br>
/// 3. `env_mask!(default = masks::Standard::default()) -> Provide a default in case of fallback <br>
/// 4. `env_mask!("MY_ENV_VAR" , masks::Standard::default()) -> Provide a default and a custom environment variable<br>
/// 5. env_mask!(value = "MY_MASK_VALUE") -> For already parsed environments, where the value represents value of the mask(say "standard(**)")<br>
macro_rules! env_mask {
    
    (value = $val : expr) => {
        mask_from_str($val , &*DEFAULT_CONVERTERS.read().unwrap())
    };
    
    () => {
        mask_from_env("GPASS_MASK", &*DEFAULT_CONVERTERS.read().unwrap())
    };

    ($mask : expr) => {
        mask_from_env($mask, &*DEFAULT_CONVERTERS.read().unwrap())
    };

    ($mask : expr, default = $default : expr) => {
        match mask_from_env($mask, &*DEFAULT_CONVERTERS.read().unwrap()) {
            Some(mask) => mask,
            None => Box::new($default),
        }
    };

    (default = $default : expr) => {
        match mask_from_env("GPASS_MASK", &*DEFAULT_CONVERTERS.read().unwrap()) {
            Some(mask) => mask,
            None => Box::new($default),
        }
    };
}

#[cfg(feature = "colored")]
use colored::Color;

#[cfg(feature = "colored")]
#[macro_export]
#[doc(hidden)]
macro_rules! env_color {
    ($var : expr) => {
        let res: Result<Color, _> = match std::env::var($var) {
            Some(val) => val.parse(),
            None => Err(()),
        };
        res
    };

    ($var : expr, $default : expr) => {
        match env_color!($var) {
            Ok(color) => color,
            Err(_) => $default,
        }
    };
}

#[cfg(feature = "colored")]
#[macro_export]
///Get the color of the mask from the environment variable `GPASS_MASK_COLOR`<br>
///The user can also specify the value of the environment variable to read <br>
/// Alternatively, can also provide a default value (default = ..)<br>
///In case the environment variable does not exist, it fallsback to the default mask color defined in colors.rs <br>
macro_rules! mask_env_color {
    () => {
        env_color!("GPASS_MASK_COLOR", crate::colors::DEFAULT_MASK_COLOR)
    };
    (default = $default : expr) => {
        env_color!("GPASS_MASK_COLOR", $default)
    };
    ($var : expr) => {
        env_color!($var, crate::colors::DEFAULT_MASK_COLOR)
    };
    ($var : expr , default = $default : expr) => {
        env_color!($var, $default)
    };
}

#[cfg(feature = "colored")]
#[macro_export]
///Macro to get prompt color from the environment using the `GPASS_PROMPT_COLOR` environment variable<br>
/// Alternatively, can also provide a default value (default = ..)<br>
macro_rules! prompt_env_color {
    () => {
        env_color!("GPASS_PROMPT_COLOR", crate::colors::DEFAULT_PROMPT_COLOR)
    };

    (default = $default : expr) => {
        env_color!("GPASS_PROMPT_COLOR", $default)
    };
    
    ($var : expr) => {
        env_color!($var, crate::colors::DEFAULT_PROMPT_COLOR)
    };
    ($var : expr , default = $default : expr) => {
        env_color!($var, $default)
    };
}
