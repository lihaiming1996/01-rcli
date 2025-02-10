use std::path::Path;

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
#[derive(Debug,Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = verify_input_filename)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long,default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

// 校验处理input参数
fn verify_input_filename(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist".to_string())
    }
}
