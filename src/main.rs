mod lib;
use lib::{execute_command, Result};

fn main() -> Result<()> {
    let result = execute_command()?;
    println!("{}", result);
    Ok(())
}
