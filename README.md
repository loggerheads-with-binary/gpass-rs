<h1>GPass:</h1>

A simple Rust library and executable bundle to accept command line password inputs and return them as a String 

* Features 
  
1. <strong>Default Output to Stderr</strong>
   
Default uses stderr for output stream, so you can view your masked password output, count characters, keep things in check without messing up any standard command flow that might require stdout (ex piping, redirection, variable store, etc)

Ex: 
```bash
$ cloud_storage_password="$(gpass --mask "or(#)" --prompt "Enter password for storage here: ")" 
```

2. <strong>Multiple types of masks</strong> 

Everyone has a different choice of how they want to check their password output. GPass provides five different types of masks out of the box

| **Mask Name** | **Comments** | **Feature** |
|:---:|:---:|:---:|
| gpass::masks::Standard |  | Simple JS-like mask using a string mask, say "\*", so input password will be masked as "\*\*\*\*\*\*\*\*" |
| gpass::masks::Echo |  | No mask cover. This might be useful if you want to see the password as you type it in. |
| gpass::masks::Blind |  | No output at all. Similar to `bash read` |
| gpass::masks::OneReverse | Feature: <br>reverse | Similar to standard mask, but the last character is shown in plaintext. Ex: password will be masked as p, \*a, \*\*s, \*\*\*s, \*\*\*\*w, \*\*\*\*\*o, \*\*\*\*\*\*r, \*\*\*\*\*\*\*d as the characters are typed in |
| gpass::masks::MimiReverse | Feature:<br>reverse | Exactly the same as OneReverse, but also works backwards(when backspaces are used) |


3. <strong>Traits for the brave</strong> 

The associated library contains traits that can be used to implement the following: 

| **Trait**          | **Feature**                          |
|--------------------|--------------------------------------|
| gpass::InputStream | To implement custom input sources    |
| gpass::Mask        | To implement custom masking behavior |
| std::io::Write     | To implement custom output sources   |

So for example, you as the user implement 
```rust
impl gpass::InputStream for MyInput;
impl gpass::Mask for MyMask;
impl std::io::Write for MyStream; 
```

You can then use: 

```rust
let gp = GPass{
    input_stream : Box::new(MyInput::new("whatever")),
    mask : Box::new(MyMask::default()), //Necessary implementation due to trait
    output_stream : Box::new(MyStream::new("whatever")),
    ..Default::default()
}
match gp.get_password(){
    Ok(password) => {println!("Password = {:?}", password);},
    Err(e) => {}
};  
```

4. <strong>Environment Macros</strong> 

This provides control in terms of masks and prompts using environment variables. The user can set the environment variables to get masks and prompts for input. 

```bash
$ GPASS_MASK="or(#)" #OneReverse with masked string '#'
$ GPASS_PROMPT="Enter the super secret password here"
$ ./your-binary-with-gpass --your-cli-flags 
Enter the super secret password here: ###### 
``` 

can be accomplished using 

```rust 
use gpass::env::*; 
let prompt = env_prompt!();
let mask = env_mask!();

if mask.is_none(){
    //Error handling
}

let mask = mask.unwrap(); 

let gp = GPass{
    prompt : prompt, 
    mask : mask,
    ..Default::default()
}; 
let password = gp.get_password()?;

```

### **Program in Action** 

Different Masks: 

1. Standard 
2. Echo 
3. Blind 
4. OneReverse
5. MimiReverse


### **Contributing** 

Just submit a pull request with whatever features you wish to add. Also advised that if you wish to implement a feature fundamentally contrasting with the library, add a feature flag. 

You can also just fork the library if you wish to make major changes 

### **License** 

[MIT](https://choosealicense.com/licenses/mit/#)

### **Bugs/Issues**

Please raise an issue or mail me personally [dev@aniruddh.ml](mailto:dev@aniruddh.ml)

