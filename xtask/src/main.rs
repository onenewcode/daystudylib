use clap::Args;
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

// 用于映射参数的结构体
#[derive(Args, Default)]
struct InferenceArgs {
    /// Model directory.
    #[clap(short, long)]
    model: String,
    /// Model type, maybe "llama", "mixtral", "llama" by default.
    #[clap(long)]
    model_type: Option<String>,

    /// Log level, may be "off", "trace", "debug", "info" or "error".
    #[clap(long)]
    log: Option<String>,

    /// Random sample temperature.
    #[clap(long)]
    temperature: Option<f32>,
    /// Random sample top-k.
    #[clap(long)]
    top_k: Option<usize>,
    /// Random sample top-p.
    #[clap(long)]
    top_p: Option<f32>,

    /// Select turbo hardware, the format is "ty:detail".
    #[clap(long)]
    turbo: Option<String>,
}

#[derive(PartialEq)]
enum ModelType {
    Llama,
    Mixtral,
}

impl InferenceArgs {
}
