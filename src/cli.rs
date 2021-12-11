use crate::args;
use std::io::Error;

pub fn simple(name: String, readonly: bool) -> anyhow::Result<()> {
    println!("Hello, {}, Readonly: {}", name, readonly);
    Ok(())
}

pub fn match_cmds(args: args::Arguements) -> anyhow::Result<()> {
    let cmd = &args.action;
    let name = args.clone().name;
    let readonly = args.clone().readonly();
    match &*cmd.to_lowercase() {
        "simple" => simple(name, readonly)?,
        _ => {
            return Err(anyhow::Error::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unknown Command",
            )))
        }
    }
    Ok(())
}
