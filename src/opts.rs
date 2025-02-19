use std::{fmt,path::Path, str::FromStr};

use clap::Parser;
// 定义命令行主结构体
#[derive(Debug,Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}
#[derive(Debug,Parser)]
pub enum Subcommand {
    #[command(name = "csv",about = "Show Csv, or convert CSV to other formats")]
    Csv(CsvOpts),

}

#[derive(Debug,Clone,Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    //Toml,
}
#[derive(Debug,Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = verify_input_filename)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long,value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long,default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 校验处理input参数
fn verify_input_filename(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            //OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            //"toml" => Ok(OutputFormat::Toml),
            v => Err(anyhow::anyhow!("Invalid format: {}", v)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
    
}
