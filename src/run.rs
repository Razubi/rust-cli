use crate::args;
use crate::cli;

pub fn run(args: args::Arguements) -> anyhow::Result<()> {
    cli::match_cmds(args)?;
    Ok(())
}
