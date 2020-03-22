mod cli;
use cli::Result;

fn main() -> Result<()> {
    let command_line_options = cli::options_from_args();
    let result = cli::execute_command(command_line_options)?;
    println!("{}", result);
    Ok(())
}
