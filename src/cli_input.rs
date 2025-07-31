use std::{error::Error, path::Path};

use clap::{Parser, Subcommand};

use crate::file_service::read_file;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encrypt {
        #[clap(flatten)]
        cmd: CryptographyCommand,
    },
    Decrypt {
        #[clap(flatten)]
        cmd: CryptographyCommand,

        #[clap(short, long)]
        salt: String,

        #[clap(short, long)]
        nonce: String,
    },
    Hash {
        #[clap(short, long, value_enum)]
        algorithm: HashAlgorithm,

         #[clap(flatten)]
        input_type: InputType
        
    },
}

#[derive(Debug, clap::Args)]
pub struct CryptographyCommand {
    #[clap(short, long, value_enum)]
    algorithm: CryptographyAlgorithm,

    #[clap(short, long)]
    pub password: String,

    #[clap(flatten)]
    pub input_type: InputType,

    #[clap(short, long)]
    pub output: String
}

#[derive(Debug, clap::Args)]
pub struct InputType {
    #[clap(short, long)]
    pub text: Option<String>,

    #[clap(short, long)]
    pub file: Option<String>,
}

impl InputType {
    pub fn validate(&self) -> Result<(), String> {
        if self.text.is_none() && self.file.is_none() {
            Err("Either --text or --file must be provided".into())
        } else {
            Ok(())
        }
    }


    pub fn extract_input_type_content(&self) -> Result<String, Box<dyn Error>> {
        self.validate()?;

        if let Some(ref text) = self.text {
            Ok(text.clone())
        } else if let Some(ref file_path) = self.file {
            let content = read_file(Path::new(&file_path))
                .map_err(|e| format!("Can`t read file content - Path: {}; Error: {}", file_path, e))?;
            Ok(content)
        } else {
            Err("Use --text or --file Flag".into())
        }
    }


}

#[derive(
    clap::ValueEnum, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug
)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
}


#[derive(
    clap::ValueEnum, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug
)]
pub enum CryptographyAlgorithm {
    Aes256Gcm
}

