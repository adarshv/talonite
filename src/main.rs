use rand::Rng;
use clipboard::{ClipboardContext, ClipboardProvider};
use termion::{color, style};
use std::env;

// Define the `main` function, which is the entry point for the program
fn main() {
    // Collect the command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();
    // Parse the first argument as a `usize` (the length of the desired password), or default to 12
    let password_length = args.get(1).and_then(|l| l.parse().ok()).unwrap_or(12);
    // Generate a random password of the specified length
    let password = generate_password(password_length);
    // Copy the password to the system clipboard
    copy_to_clipboard(&password).unwrap();
    // Print a message indicating that the password has been copied to the clipboard, with bold green text
    println!("Copied password {} to the clipboard.", format!("{}{}{}{}", style::Bold, color::Fg(color::Green), password, style::Reset));
}

// Define the `generate_password` function, which generates a random password of the specified length
fn generate_password(length: usize) -> String {
    // Define the set of symbols to choose from
    let symbols = "!@#$%^&*?";
    // Define the set of ambiguous characters to exclude from the symbol set
    let ambiguous_chars = "O0l1I";
    // Remove the ambiguous characters from the symbol set
    let symbols = symbols.chars().filter(|c| !ambiguous_chars.contains(*c)).collect::<String>();
    // Initialize an empty string to hold the password
    let mut password = String::new();
    // Initialize a random number generator
    let mut rng = rand::thread_rng();
    // Add at least two symbols to the password
    for _ in 0..2 {
        // Choose a random symbol from the symbol set and add it to the password
        let symbol = symbols.chars().nth(rng.gen_range(0..symbols.len())).unwrap();
        password.push(symbol);
    }
    // Add random letters, digits, and symbols to the password until it reaches the desired length
    while password.len() < length {
        // Choose a random character type (letter, digit, or symbol)
        let char_type = rng.gen_range(0..3);
        // Add a random character of the chosen type to the password
        let char = match char_type {
            // Add a random lowercase letter
            0 => (rng.gen_range(b'a'..=b'z') as char).to_string(),
            // Add a random digit
            1 => rng.gen_range(0..=9).to_string(),
            // Add a random symbol
            _ => symbols.chars().nth(rng.gen_range(0..symbols.len())).unwrap().to_string(),
        };
        password.push_str(&char);
    }
    // Return the generated password
    password
}

// Define the `copy_to_clipboard` function, which copy generated password to system clipboard
fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a mutable variable ctx of type ClipboardContext
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    // Set the contents of the clipboard to the given text by calling the set_contents method on the ctx object.
    ctx.set_contents(text.to_owned())?;
    // Return an empty tuple wrapped in an Ok value to indicate success
    Ok(())
}
