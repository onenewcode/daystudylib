use clap::Subcommand;
use clap::Parser;
// 通过.cargo中config.toml中配置[alias]中
fn main() {
    match Cli::parse().command {
        Commands::ListTurbo => {
          println!("ListTurbo")
        }
        Commands::Deploy => {
            println!("Deploy")
        }
        Commands::Cast => {
           println!("Cast")
        }
        Commands::Generate => {
           println!("Generate")
        }
        Commands::Chat => {
           println!("Chat")
        }
        Commands::Service => {
           println!("Service")
        }
    }
}
#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    ListTurbo,
    Deploy,
    Cast,
    Generate,
    Chat,
    Service,
}

