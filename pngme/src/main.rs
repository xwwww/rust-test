#![allow(unused_variables)]
use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;
use std::str::FromStr;
use pngme::png::Png;  // 修改为使用库名引用
use pngme::chunk::Chunk;  // 修改为使用库名引用
use pngme::chunk_type::ChunkType;  // 修改为使用库名引用
use std::fs::File;
use std::io::{Read, Write};

// 定义命令行参数的枚举和结构体
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(clap::Parser, Debug)]
pub struct EncodeArgs {
    /// 输入的 PNG 文件路径
    // #[clap(parse(from_os_str))]
    #[clap(value_parser)]
    pub file_path: PathBuf,
    /// 数据块类型
    pub chunk_type: String,
    /// 要编码的消息
    pub message: String,
    /// 可选的输出文件路径
    #[clap(short = 'o', long = "output", value_parser)]
    pub output: Option<PathBuf>,
}

#[derive(clap::Parser, Debug)]
pub struct DecodeArgs {
    /// 输入的 PNG 文件路径
    // #[clap(parse(from_os_str))]
    pub file_path: PathBuf,
    /// 数据块类型
    pub chunk_type: String,
}

#[derive(clap::Parser, Debug)]
pub struct RemoveArgs {
    /// 输入的 PNG 文件路径
    // #[clap(parse(from_os_str))]
    pub file_path: PathBuf,
    /// 数据块类型
    pub chunk_type: String,
}

#[derive(clap::Parser, Debug)]
pub struct PrintArgs {
    /// 输入的 PNG 文件路径
    // #[clap(parse(from_os_str))]
    pub file_path: PathBuf,
}

impl PngMeArgs {
    // 解析命令行参数的方法
    pub fn from_args() -> Self {
        let args = Cli::parse();
        match args.command {
            Commands::Encode(args) => PngMeArgs::Encode(args),
            Commands::Decode(args) => PngMeArgs::Decode(args),
            Commands::Remove(args) => PngMeArgs::Remove(args),
            Commands::Print(args) => PngMeArgs::Print(args),
        }
    }
}

// 命令行接口结构体
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

// 命令枚举
#[derive(Parser, Debug)]
enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

fn main() -> Result<()> {
    let args = PngMeArgs::from_args();
    match args {
        PngMeArgs::Encode(args) => {
            let file_path_clone = args.file_path.clone();
            let mut file = File::open(file_path_clone)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let mut png = Png::try_from(buffer.as_slice())?;
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let chunk = Chunk::new(chunk_type, args.message.into_bytes());
            png.append_chunk(chunk);
            let output_path = args.output.unwrap_or_else(|| args.file_path.clone());
            let mut output_file = File::create(output_path)?;
            output_file.write_all(&png.as_bytes())?;
            println!("Message encoded successfully.");
        }
        PngMeArgs::Decode(args) => {
            let mut file = File::open(args.file_path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let png = Png::try_from(buffer.as_slice())?;
            if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
                if let Ok(message) = chunk.data_as_string() {
                    println!("Decoded message: {}", message);
                } else {
                    println!("Failed to decode message as valid UTF-8.");
                }
            } else {
                println!("Chunk of type {} not found.", args.chunk_type);
            }
        }
        PngMeArgs::Remove(args) => {
            let mut file = File::open(args.file_path.clone())?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let mut png = Png::try_from(buffer.as_slice())?;
            if let Ok(removed_chunk) = png.remove_first_chunk(&args.chunk_type) {
                let mut output_file = File::create(args.file_path)?;
                output_file.write_all(&png.as_bytes())?;
                println!("Chunk of type {} removed successfully.", args.chunk_type);
            } else {
                println!("Chunk of type {} not found.", args.chunk_type);
            }
        }
        PngMeArgs::Print(args) => {
            let mut file = File::open(args.file_path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            let png = Png::try_from(buffer.as_slice())?;
            for chunk in png.chunks() {
                println!("{}", chunk);
            }
        }
    }
    Ok(())
}

