mod args;
mod cli;
mod run;

pub use anyhow::Context;
pub use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    run::run(args::Arguements::from_args())?;
    Ok(())
}
