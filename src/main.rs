use color_eyre::eyre::Result;
use structopt::StructOpt;


/// A CLI for a digital garden
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// write something in your garden
    /// 
    /// This command will open your $Editor, wait for you
    /// to write something, and then save the file to your
    /// garden
    Write {
        /// Optionally set a title for what you are going to write
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn main() -> Result<()>{
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(opt);
    todo!();
}
