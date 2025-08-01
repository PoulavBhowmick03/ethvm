use std::{
    io,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ethvm")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // list
    Ls,
    // will add Install, Use, Rm later
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Ls => {
            let path = prepare_install_dir()?;
            let versions = list_versions(&path);
            let current = current_version(&path)?;
            for v in versions? {
                if let Some(ref v) = current {
                    println!("*{:?}", v);
                }
                println!("{:?}", v);
            }
        }
    }
    Ok(())
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

fn current_version(install_dir: &Path) -> io::Result<Option<String>> {
    let symlink_path = install_dir.join("current");
    let meta = std::fs::symlink_metadata(&symlink_path).unwrap();
    if !meta.file_type().is_symlink() {
        return Ok(None);
    }
    let target = match std::fs::read_link(&symlink_path) {
        Ok(p) => p,
        Err(_) => return Ok(None),
    };

    let os_name = match target.file_name() {
        Some(n) => n,
        None => return Ok(None),
    };
    let final_path = match os_name.to_str() {
        Some(s) => s,
        None => return Ok(None),
    };

    Ok(Some(final_path.to_string()))
}
