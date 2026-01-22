use melies_rust;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    /// Turn debugging information on
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Get all of the players in a demo
    GetPlayers {
        /// path to the demo
        demo: String,
        /// path to the output
        #[arg(short, long, default_value_t = String::from(""))]
        output: String
    },
    /// Get the chat from a demo
    GetChat {
        /// path to the demo
        demo: String,
        /// path to the output
        #[arg(short, long, default_value_t = String::from(""))]
        output: String
    },
    /// Get both the players and the chat
    GetInfo {
        /// path to the demo
        demo: String,
        /// path to the output
        #[arg(short, long, default_value_t = String::from(""))]
        output: String
    }
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::GetPlayers { demo, output } => {
            melies_rust::demos::get_players(&demo, &output);
        },
        Commands::GetChat { demo, output } => {
            melies_rust::demos::get_chat(&demo, &output);
        }
        Commands::GetInfo { demo, output } => {
            melies_rust::demos::get_info(&demo, &output);
        }
    }
}