use crate::*;

#[test]
fn test_mask() -> Result<(), Box<dyn std::error::Error>> {
    let mask = masks::Standard::new(masks::DEFAULT_MASK_CHAR);
    let mut gpass = GPass {
        input_stream: Box::new(Getch::new()),
        mask: Box::new(mask),
        output_stream: Box::new(std::io::stderr()),
        ctrl_c_abort: false,
        prompt: "Enter Password Here: ".to_string(),
    };

    gpass.ctrl_c_abort = true;
    Ok(())
}

#[test]
fn test_input_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let password = "Hello World!".to_string();
    let input_stream = crate::file_input::IString::new(password.clone());
    let output_stream = std::io::stderr();

    let gpass = GPass::new(
        Some("Enter Password Hello World! Here: "),
        Box::new(input_stream),
        Box::new(masks::Echo::default()),
        Box::new(output_stream),
        false,
    );

    let password2 = gpass.get_password()?;
    assert_eq!(password, password2);

    Ok(())
}

#[test]
fn test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::BufReader;

    //Write input.txt with "Hello World!";
    let file = "input.txt";
    let password = "Hello World!";
    std::fs::write(file, password)?;

    let input_stream = BufReader::new(std::fs::File::open(file)?);

    let gpass = GPass::new(
        Some("Enter Password Hello World! Here: "),
        Box::new(input_stream),
        Box::new(masks::Echo::default()),
        Box::new(std::io::stderr()),
        false,
    );

    let password2 = gpass.get_password()?;
    assert_eq!(password, password2);
    Ok(())
}
