mod demos;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Add a Get command
    GetPlayers {
        /// path to the demo
        demo: String,

        /// path to the output
        #[arg(short, long, default_value_t = String::from("players.json"))]
        output: String
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::GetPlayers { demo, output } => {
            demos::get_players(&demo, &output);
        }
    }
}