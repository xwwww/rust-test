use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;
use pngme_lib::{Png, Chunk, ChunkType};
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
struct EncodeArgs {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Parser, Debug)]
struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Parser, Debug)]
struct PrintArgs {
    file_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print_chunks(args),
    }
}

fn encode(args: EncodeArgs) -> Result<()> {
    let mut file = File::open(&args.file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut png = Png::try_from(buffer.as_slice())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.into_bytes());
    png.append_chunk(chunk);
    let output_path = args.output.unwrap_or(args.file_path);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&png.as_bytes())?;
    println!("Message encoded successfully.");
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
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
    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let mut file = File::open(&args.file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut png = Png::try_from(buffer.as_slice())?;
    if let Ok(_removed_chunk) = png.remove_first_chunk(&args.chunk_type) {
        let mut output_file = File::create(args.file_path)?;
        output_file.write_all(&png.as_bytes())?;
        println!("Chunk of type {} removed successfully.", args.chunk_type);
    } else {
        println!("Chunk of type {} not found.", args.chunk_type);
    }
    Ok(())
}

fn print_chunks(args: PrintArgs) -> Result<()> {
    let mut file = File::open(args.file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let png = Png::try_from(buffer.as_slice())?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}