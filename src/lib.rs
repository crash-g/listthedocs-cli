mod client;
mod command_executor;
mod command_line;
mod entities;
mod error;

pub use command_executor::execute_command;
pub use command_line::options_from_args;
pub use error::{Error, Result};
