// args.rs -  Arguement parsing

use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Clone)]
pub struct Arguements { 
    /// The command To run
    pub action: String,

    ///Argument To Pass
    pub arguements: Vec<String>,

    ///Name 
    #[structopt(long, short)]
    pub name: String,

    /// Readonly Mode
    #[structopt(long, short)]
    pub(crate) readonly: Option<Option<bool>>,
    

}
