use std::io;
use std::io::Write;

pub fn f1_input() -> io::Result<()> {
    let mut buffer = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut buffer)?;
    println!("Your name is {buffer}");

    Ok(())
}