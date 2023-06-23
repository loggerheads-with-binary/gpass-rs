use clap::Parser; 


#[derive(Parser, Debug)]
pub struct Args{
    ///Mask for the password input 
    #[clap(short, long, env = "GPASS_MASK")]
    pub mask: Option<String>, 

    ///Prompt for password input 
    #[clap(short, long, env = "GPASS_PROMPT" , default_value = "Enter Password Here: ")]
    pub prompt : String, 

    #[cfg(feature = "colored")]
    ///Color for the prompt 
    #[clap(long, env = "GPASS_PROMPT_COLOR" )]
    pub color_prompt : Option<String>, 

    #[cfg(feature = "colored")]
    ///Color for the mask 
    #[clap(long, env= "GPASS_MASK_COLOR"  )]
    pub color_mask : Option<String>,

    ///Ctrl+C does not abort, instead returns password collected until then 
    #[clap(short, long, env = "GPASS_NO_ABORT")]
    pub no_abort : bool, 

    ///Prints to Stdout instead of Stderr 
    #[clap(short, long, env = "GPASS_STDOUT")]
    pub stdout : bool  
}

pub fn get_args() -> Args{
    Args::parse()
}