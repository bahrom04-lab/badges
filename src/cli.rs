use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub owner: String,

    #[arg(short, long)]
    pub repo: String,

    #[arg(short, long)]
    pub instance: String,
}
