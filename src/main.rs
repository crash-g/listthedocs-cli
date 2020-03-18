mod lib;
use lib::{execute_command, options_from_args, Result};

fn main() -> Result<()> {
    let command_line_options = options_from_args();
    let result = execute_command(command_line_options)?;
    println!("{}", result);
    Ok(())
}
