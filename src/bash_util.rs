use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::process::{Command, Output};

pub fn run_bash_script(
    working_dir_path: &str,
    script_path: &str,
    env_vars: HashMap<String, String>,
) -> Result<Output, Box<dyn Error>> {
    // Save the current working directory
    let original_dir = env::current_dir()?;

    // Change to the working directory
    env::set_current_dir(working_dir_path)?;

    // Build the command
    let mut command = Command::new("bash");
    command.arg(script_path);

    // Set the environment variables
    for (key, value) in &env_vars {
        command.env(key, value);
    }

    // Execute the script
    let output = command.output();

    // Restore the original working directory
    env::set_current_dir(original_dir)?;

    // Return the command output or an error
    output.map_err(|e| Box::new(e) as Box<dyn Error>)
}