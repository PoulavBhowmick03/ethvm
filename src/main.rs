use std::{
    io,
    path::{Path, PathBuf},
};

use clap::{Error, Parser, builder::Str};

#[derive(Parser, Debug)]
#[command(name = "ethvm")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug)]
enum Commands {
    // list
    Ls,
    // will add Install, Use, Rm later

}

fn main() {
    let args = Cli::parse();

    for _ in 0..args.count {
        println!("Hello {}", args.name);
    }
}

fn prepare_install_dir() -> io::Result<PathBuf> {
    let home = std::env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME not set"))?;
    let install_path = PathBuf::from(home).join(".local/share/ethvm/lighthouse");

    std::fs::create_dir_all(&install_path)?;
    Ok(install_path)
}

fn list_versions(install_dir: &Path) -> io::Result<Vec<String>> {
    let mut versions = Vec::new();
    for entry_result in std::fs::read_dir(install_dir)? {
        let entry = entry_result?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                versions.push(name.to_owned());
            }
        }
    }
    versions.sort();
    Ok(versions)
}
