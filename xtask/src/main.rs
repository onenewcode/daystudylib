use clap::Args;
use clap::Subcommand;
use clap::Parser;
// 通过.cargo中config.toml中配置[alias]中
fn main() {
    match Cli::parse().command {
        Commands::Chat(a) => {
          print!("{}",a.prompt)
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
    Chat(ChatArgs),
}

// 用于映射参数的结构体
#[derive(Args, Default)]
struct ChatArgs {
    #[clap(short, long,default_value_t = 0)]
    user_id: u32,
    /// Session id.
    #[clap(short, long,default_value_t = 0)]
    session_id:u32,
    /// Model directory.
    #[clap(short, long,default_value = "cmd")]
    mode: String,
    #[clap(short, long,default_value = "system")]
    prompt: String,
}

