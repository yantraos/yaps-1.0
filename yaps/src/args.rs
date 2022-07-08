use clap::{crate_version, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "yaps")]
#[clap(author = "Prateek Sunal <prateek.sunal@acsestech.com>")]
#[clap(version = crate_version!())]
#[clap(about = "Package manager for yantra OS")]
#[clap(subcommand_required = true)]
#[clap(dont_collapse_args_in_usage = true)]
#[clap(propagate_version = true)]
pub struct YapsArgs {
    #[clap(subcommand)]
    pub task: Task,
}

#[derive(Subcommand, Debug)]
#[clap(trailing_var_arg = true)]
#[clap(dont_delimit_trailing_values = true)]
#[clap(allow_hyphen_values = true)]
pub enum Task {
    #[clap(about = "Install packages into system")]
    #[clap(allow_hyphen_values = true)]
    Install {
        package: String,

        #[clap(short, long)]
        #[clap(takes_value = false)]
        /// Skip dependencies resolving
        no_depends: bool,
    },

    #[clap(about = "Remove packages from the system")]
    Remove { package: String },

    #[clap(about = "Update repository packages database")]
    Update {},

    #[clap(about = "Print information of specified package")]
    Info { package: String },

    #[clap(about = "Perform system package updation")]
    Upgrade {
        #[clap(long)]
        /// Set Compiler specification style
        compiler_specs: Option<String>,
    },

    #[clap(about = "List all avaliable packages")]
    List {},

    #[clap(about = "Configure packages from source")]
    Compile {
        package: String,
        #[clap(long)]
        #[clap(takes_value = false)]
        /// Skip dependencies resolving
        no_depends: bool,

        #[clap(long)]
        /// Set Compiler specification style
        compiler_specs: Option<String>,

        #[clap(long)]
        #[clap(takes_value = false)]
        /// Set NOINSTALL flags to skip install while compiling
        no_install: bool,
    },
    #[clap(about = "Return list of required dependencies of specified package")]
    Depends { package: String },
}
