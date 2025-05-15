mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::chunk::Chunk;
use crate::chunk::ChunkError;
use crate::chunk_type::ChunkType;
use crate::commands as other_commands;
use crate::png::Png;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pngme", version, about = "A PNG steganography CLI")]
struct Cli {
    #[command(subcommand)]
    command: ParsedCommands,
}

#[derive(Subcommand)]
enum ParsedCommands {
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
    },
    Decode {
        file_path: String,
        chunk_type: String,
    },
    Remove {
        file_path: String,
        chunk_type: String,
    },
    Print {
        file_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        ParsedCommands::Encode {
            file_path,
            chunk_type,
            message,
        } => {
            other_commands::encode(&file_path, &chunk_type, &message);
        }
        ParsedCommands::Decode {
            file_path,
            chunk_type,
        } => {
            other_commands::decode(&file_path, &chunk_type);
        }
        ParsedCommands::Remove {
            file_path,
            chunk_type,
        } => {
            other_commands::remove(&file_path, &chunk_type);
        }
        ParsedCommands::Print { file_path } => {
            other_commands::print_chunks(&file_path);
        }
    }
}
