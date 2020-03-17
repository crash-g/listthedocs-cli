use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// The URL to of the ListTheDocs service (e.g., http://localhost:5000)
    #[structopt(short, long, env = "DOCS_URL")]
    pub url: Option<String>,

    /// The API key to use for authentication
    #[structopt(short, long, env = "DOCS_API_KEY", hide_env_values = true)]
    pub api_key: Option<String>,

    /// Path to a json file containing the URL and, optionally, the API key.
    /// If given, this takes priority over the other options.
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// The output is JSON in case of success, unless this flag is
    /// given, in which case the output will be nicely formatted
    /// to make it easier to read.
    #[structopt(short, long)]
    pub pretty_print: bool,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Project {
        #[structopt(subcommand)]
        project_command: ProjectCommand,
    },

    Version {
        #[structopt(subcommand)]
        version_command: VersionCommand,
    },

    User {
        #[structopt(subcommand)]
        user_command: UserCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum ProjectCommand {
    /// Add a new project
    Add {
        /// The title of the project
        #[structopt(long)]
        title: Option<String>,

        /// A short description
        #[structopt(long)]
        description: Option<String>,

        /// A link to a logo
        #[structopt(long)]
        logo: Option<String>,

        /// Path to a json file containing the definition of the project to add.
        /// If given, this takes priority over the other options.
        #[structopt(parse(from_os_str))]
        file_path: Option<PathBuf>,
    },

    /// Get an existing project
    Get {
        /// The code of the project, as returned upon insertion
        code: String,
    },

    /// Get the list of all projects
    GetAll,

    /// Update an existing project
    Update {
        /// The code of the project, as returned upon insertion
        #[structopt(long)]
        code: Option<String>,

        /// A short description
        #[structopt(long)]
        description: Option<String>,

        /// A link to a logo
        #[structopt(long)]
        logo: Option<String>,

        /// Path to a json file containing the definition of the project to update.
        /// If given, this takes priority over all other options.
        #[structopt(parse(from_os_str))]
        file_path: Option<PathBuf>,
    },

    /// Remove a project
    Remove {
        /// The code of the project, as returned upon insertion
        code: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum VersionCommand {
    Todo {},
}

#[derive(Debug, StructOpt)]
pub enum UserCommand {
    Add {
        /// The name of the user
        #[structopt(long)]
        name: Option<String>,

        #[structopt(long)]
        is_admin: Option<bool>,

        /// Path to a json file containing the definition of the user to add.
        /// If given, this takes priority over all other options.
        #[structopt(parse(from_os_str))]
        file_path: Option<PathBuf>,
    },
}

pub fn opt_from_args() -> Opt {
    Opt::from_args()
}
