#![allow(unused_imports)]
#![allow(unused_mut)]

use gpass::*;
use gpass::env::*; 
use eyre::{Result, WrapErr}; 
use std::sync::Arc;


mod user_args; 

fn main() -> Result<()> {

    let args = user_args::get_args();

    let mut mask : Box<dyn Mask> = match args.mask{
        None => Box::new(masks::Standard::default()),
        Some(mask) => env_mask!(value = &mask).ok_or_else(|| LibError::Other("Invalid Mask Value".into())).wrap_err("Mask could not be generated at runtime")?
    }; 

    #[cfg(feature = "colored")]
    mask.set_color(match args.color_mask{
        Some(ref val) => &val,
        None => colors::DEFAULT_MASK_COLOR
    });

    let mut output_stream : Box<dyn std::io::Write> = match args.stdout{
        false => Box::new(std::io::stderr()),
        true => Box::new(std::io::stdout())
    };

    let gp = GPass{
        mask, 
        output_stream, 
        ctrl_c_abort : !args.no_abort,
        prompt : args.prompt, 

        #[cfg(feature = "colored")]
        prompt_color : Arc::from(match args.color_prompt{
            Some(ref val) => val,
            None => colors::DEFAULT_PROMPT_COLOR
        }),    

        ..Default::default()
    };
    
    let password = gp.get_password().wrap_err("Failed to obtain password")?;
    print!("{password}");
    Ok(())
}
