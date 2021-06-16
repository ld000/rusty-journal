mod cli;
mod tasks;

use anyhow::anyhow;
use structopt::StructOpt;
use crate::cli::{CommandLineArgs, Action};
use crate::tasks::Task;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = cli::CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Action::Add { text } => tasks::add_task(journal_file, Task::new(text)),
        Action::Done { position } => tasks::complete_task(journal_file, position),
        Action::List => tasks::list_tasks(journal_file),
    }?;

    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}