mod args;
mod constants;
mod structs;
mod commands {
    pub mod compile;
    pub mod depends;
    pub mod info;
    pub mod install;
    pub mod list;
    pub mod remove;
    pub mod update;
    pub mod upgrade;
}
mod functions {
    pub mod get_arch;
    pub mod get_deps;
    pub mod is_installed;
    pub mod list_depends;
    pub mod read_input;
    pub mod run_tar;
    pub mod y_builder;
    pub mod y_compiler;
    pub mod y_downloader;
    pub mod y_info;
    pub mod y_installer;
    pub mod yy_build_path;
}

use args::{Task, YapsArgs};
use clap::Parser;

fn main() {
    let args = YapsArgs::parse();

    // Depending on what subcommand the user has put in the CLI, we call the related function.
    match &args.task {
        Task::Install {
            package,
            no_depends,
        } => commands::install::install(package.to_string(), no_depends),
        Task::Remove { package } => commands::remove::remove(package.to_string()),
        Task::Upgrade { compiler_specs } => commands::upgrade::upgrade(compiler_specs),
        Task::Update {} => commands::update::update(),
        Task::Info { package } => commands::info::info(package.to_string()),
        Task::Depends { package } => commands::depends::depends(package.to_string()),
        Task::List {} => commands::list::list(),
        Task::Compile {
            package,
            no_depends,
            no_install,
            compiler_specs,
        } => {
            commands::compile::compile(package.to_string(), no_depends, no_install, compiler_specs)
        }
    }
}
