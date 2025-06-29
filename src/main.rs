/// Cross‑platform SSH CLI application.
///
/// This application provides a command-line interface for managing SSH connection entries,
/// supporting multiple authentication modes, configuration import/export, and synchronization
/// between devices. The main features include:
///
/// - Adding, updating, removing, and listing SSH entries
/// - Connecting to SSH hosts using stored credentials
/// - Exporting and importing configuration files
/// - Synchronizing configuration with other devices over the network

mod config;
mod ssh_client;
mod sync;

use clap::{Parser, Subcommand};
use config::{encode_clear, AuthMode, Config, ConfigManager, Entry};

#[derive(Parser)]
#[command(name = "ssh_cli", about = "Cross‑platform SSH CLI")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add or update an entry
    Add {
        name: String,
        ip: String,
        username: String,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        rsa: Option<String>,
        #[arg(value_parser = ["password", "rsa", "both"])]
        auth: String,
    },
    /// Remove an entry
    Remove { name: String },
    /// List all entries
    List,
    /// Connect to an entry
    Connect { name: String },
    /// Export config to a path
    Export { path: String },
    /// Import config from a path
    Import { path: String },
    /// Sync send <target‑ip>
    SyncSend { target_ip: String },
    /// Sync recv (blocks)
    SyncRecv,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cm = ConfigManager::new()?;
    let mut cfg = cm.load()?;

    match cli.cmd {
        Commands::Add {
            name,
            ip,
            username,
            password,
            rsa,
            auth,
        } => {
            let auth_mode = match auth.as_str() {
                "password" => AuthMode::Password,
                "rsa" => AuthMode::Rsa,
                "both" => AuthMode::Both,
                _ => unreachable!(),
            };
            let entry = Entry {
                name: name.clone(),
                ip,
                username,
                password: password.map(|p| encode_clear(&p)),
                rsa_key: rsa.map(|k| encode_clear(&k)),
                auth_mode,
            };

            if let Some(e) = cfg.find_mut(&name) {
                *e = entry;
                println!("Updated entry {name}");
            } else {
                cfg.entries.push(entry);
                println!("Added entry {name}");
            }
            cm.save(&cfg)?;
        }
        Commands::Remove { name } => {
            cfg.entries.retain(|e| e.name != name);
            cm.save(&cfg)?;
            println!("Removed {name}");
        }
        Commands::List => {
            for e in &cfg.entries {
                println!(
                    "{} @ {} ({:?})",
                    e.name,
                    e.ip,
                    e.auth_mode
                );
            }
        }
        Commands::Connect { name } => {
            let entry = cfg
                .find(&name)
                .ok_or_else(|| anyhow::anyhow!("no such entry"))?
                .clone();
            ssh_client::connect(&entry)?;
        }
        Commands::Export { path } => {
            std::fs::copy(cm.path(), &path)?;
            println!("Exported to {}", path);
        }
        Commands::Import { path } => {
            let imported: Config =
                serde_json::from_str(&std::fs::read_to_string(&path)?)?;
            cfg = imported;
            cm.save(&cfg)?;
            println!("Imported from {}", path);
        }
        Commands::SyncSend { target_ip } => {
            let json = std::fs::read_to_string(cm.path())?;
            sync::send(json, &target_ip)?;
        }
        Commands::SyncRecv => {
            sync::recv(cm.path().clone())?;
        }
    }
    Ok(())
}
