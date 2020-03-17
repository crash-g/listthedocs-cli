use std::path::PathBuf;
use std::str::FromStr;
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

    /// The output is human-readable by default. Use this option for JSON.
    #[structopt(short, long)]
    pub json: bool,

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

    Role {
        #[structopt(subcommand)]
        role_command: RoleCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum ProjectCommand {
    /// Add a new project
    Add {
        /// The title of the project
        title: Option<String>,

        /// A short description
        description: Option<String>,

        /// A link to a logo
        logo: Option<String>,

        /// Path to a json file containing the definition of the project to add.
        /// If given, no arguments must be passed.
        #[structopt(short, parse(from_os_str))]
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
        code: String,

        /// A short description
        #[structopt(long = "desc")]
        description: Option<String>,

        /// A link to a logo
        #[structopt(long)]
        logo: Option<String>,

        /// Path to a json file containing the definition of the project to update.
        /// If given, no arguments must be passed.
        #[structopt(short, parse(from_os_str))]
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
        name: Option<String>,

        /// Whether the use is an admin or not
        is_admin: Option<bool>,

        /// Path to a json file containing the definition of the user to add.
        /// If given, no arguments must be passed.
        #[structopt(short, parse(from_os_str))]
        file_path: Option<PathBuf>,
    },

    Get {
        /// The name of the user
        name: String,
    },

    /// Get the list of all users
    GetAll,
}

#[derive(Debug, StructOpt)]
pub enum RoleCommand {
    Add {
        /// The name of the user
        user_name: String,

        /// A list of roles to add
        project_role: Vec<ProjectRole>,

        /// Path to a json file containing the definition of the roles to add.
        /// If given, no arguments must be passed.
        #[structopt(short, parse(from_os_str))]
        file_path: Option<PathBuf>,
    },

    Remove {
        /// The name of the user
        user_name: String,

        /// A list of roles to remove
        project_role: Vec<ProjectRole>,

        /// Path to a json file containing the definition of the roles to remove.
        /// If given, no arguments must be passed.
        #[structopt(short, parse(from_os_str))]
        file_path: Option<PathBuf>,
    },

    Get {
        /// The name of the user
        user_name: String,
    },
}

#[derive(Debug)]
pub struct ProjectRole {
    pub role_name: String,
    pub project_code: String,
}

impl FromStr for ProjectRole {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("/").collect::<Vec<_>>()[..] {
            [role_name, project_code] => Ok(ProjectRole {
                role_name: role_name.to_owned(),
                project_code: project_code.to_owned(),
            }),
            _ => Result::Err(format!("Invalid project role: {}", s)),
        }
    }
}

pub fn opt_from_args() -> Opt {
    Opt::from_args()
}
