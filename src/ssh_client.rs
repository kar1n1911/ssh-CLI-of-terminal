use crate::config::{decode_clear, AuthMode, Entry};
use ssh2::Session;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    process::Command,
};
use tempfile::NamedTempFile;

/// Try native `ssh` first (with temp key), else fall back to `ssh2`.
pub fn connect(entry: &Entry) -> anyhow::Result<()> {
    // 1️⃣  Attempt to spawn system ssh (gives users the same UX they know).
    if let Ok(_) = which::which("ssh") {
        return spawn_system_ssh(entry);
    }

    // 2️⃣  Embedded / library fallback.
    connect_via_ssh2(entry)
}

/// Spawn local `ssh` command, writing RSA key (if any) into a temp file.
fn spawn_system_ssh(entry: &Entry) -> anyhow::Result<()> {
    let mut cmd = Command::new("ssh");
    cmd.arg(format!("{}@{}", entry.username, entry.ip));

    // Handle auth modes
    match entry.auth_mode {
        AuthMode::Password => {
            // Ask password interactively (system ssh handles it)
        }
        AuthMode::Rsa | AuthMode::Both => {
            let key_data = entry
                .rsa_key
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No RSA key stored"))?;
            let key_dec = decode_clear(key_data)?;
            let mut temp = NamedTempFile::new()?;
            temp.write_all(key_dec.as_bytes())?;
            cmd.arg("-i").arg(temp.path());
            // We keep `temp` alive until ssh exits
            let status = cmd.status()?;
            if !status.success() {
                anyhow::bail!("ssh exited with {}", status);
            }
            return Ok(());
        }
    };

    // For password‑only we just run ssh as is.
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("ssh exited with {}", status);
    }
    Ok(())
}

/// Pure‑Rust fallback
fn connect_via_ssh2(entry: &Entry) -> anyhow::Result<()> {
    let tcp = TcpStream::connect(format!("{}:22", entry.ip))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    match entry.auth_mode {
        AuthMode::Password => {
            let pwd = decode_clear(
                entry
                    .password
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("password missing"))?,
            )?;
            sess.userauth_password(&entry.username, &pwd)?;
        }
        AuthMode::Rsa => {
            let key_dec = decode_clear(
                entry
                    .rsa_key
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("rsa key missing"))?,
            )?;
            sess.userauth_pubkey_memory(&entry.username, None, &key_dec, None)?;
        }
        AuthMode::Both => {
            let key_ok = if let Some(k) = &entry.rsa_key {
                let dk = decode_clear(k)?;
                sess.userauth_pubkey_memory(&entry.username, None, &dk, None)
                    .is_ok()
            } else {
                false
            };
            if !key_ok {
                let pwd = decode_clear(
                    entry
                        .password
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("password missing"))?,
                )?;
                sess.userauth_password(&entry.username, &pwd)?;
            }
        }
    }

    if !sess.authenticated() {
        anyhow::bail!("Authentication failed");
    }

    // Simple interactive shell piping
    let mut channel = sess.channel_session()?;
    channel.request_pty("xterm", None, None)?;
    channel.shell()?;

    let mut ch_stdout = channel.stream(0);
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut buffer = [0u8; 1024];
        if ch_stdout.read(&mut buffer).is_ok() {
            stdout.write_all(&buffer)?;
            stdout.flush()?;
        }
        let mut input = [0u8; 1024];
        if stdin.read(&mut input).is_ok() {
            channel.write_all(&input)?;
            channel.flush()?;
        }
        if channel.eof() {
            break;
        }
    }

    channel.close()?;
    Ok(())
}
