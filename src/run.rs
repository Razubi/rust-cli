use crate::cli;
use crate::args;

pub fn run(args: args::Arguements) -> anyhow::Result<()>{
    cli::match_cmds(args)?;
    Ok(())
}