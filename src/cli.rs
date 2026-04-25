use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short = 'f', long = "file", required = true, help = "配置文件路径")]
    pub config_file: String,
    
    #[arg(short = 'v', long = "verbose", help = "详细输出")]
    pub verbose: bool,
}