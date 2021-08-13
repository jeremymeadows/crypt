use crate::stdin_extras::read_hidden::ReadHidden;
use std::io::{self, Stdin, Write};

/// Provides methods to simultaneously print a prompt message to `Stdout` as well as read the
/// response from `Stdin`.
pub trait Input {
    fn input(&self, prompt: &str) -> io::Result<String>;
    fn input_hidden(&self, prompt: &str) -> io::Result<String>;
}

impl Input for Stdin {
    /// Prints `prompt`, then returns the user's input.
    fn input(&self, prompt: &str) -> io::Result<String> {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();

        let mut val = String::new();
        match io::stdin().read_line(&mut val) {
            Ok(_) => Ok(String::from(val.trim())),
            Err(e) => Err(e),
        }
    }

    /// Prints `prompt`, then returns the user's input without echoing to `Stdout`.
    fn input_hidden(&self, prompt: &str) -> io::Result<String> {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();

        let mut val = String::new();
        match io::stdin().read_hidden_line(&mut val) {
            Ok(_) => Ok(String::from(val.trim())),
            Err(e) => Err(e),
        }
    }
}
