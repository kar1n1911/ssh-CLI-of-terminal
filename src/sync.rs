//! Very small “sync over IP” mechanism using tiny_http + ureq.
//! Run `ssh_cli sync recv` on the target machine first,
//! then  `ssh_cli sync send <TARGET_IP>` on the source.

use std::{fs, path::PathBuf};

use tiny_http::{Method, Response, Server};

/// Send our local JSON config to the target machine.
pub fn send(cfg_json: String, target_ip: &str) -> anyhow::Result<()> {
    let url = format!("http://{}:8080/receive", target_ip);

    // ureq::Request has `send_string`; we just added the crate above.
    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .send_string(&cfg_json)?;

    if resp.status() >= 400 {
        anyhow::bail!("sync failed ‑ server said {}", resp.status());
    }
    println!("Config sent ✅");
    Ok(())
}

/// Block forever, serving GET / (download) and POST /receive (upload).
pub fn recv(path: PathBuf) -> anyhow::Result<()> {
    println!("Receiving on 0.0.0.0:8080 …  (Ctrl‑C to stop)");
    let server = Server::http("0.0.0.0:8080").map_err(anyhow::Error::msg)?;
    for mut request in server.incoming_requests() {
        // NOTE: `mut request` so we can call the &mut method `as_reader`.
        let resp = match (request.method(), request.url()) {
            (&Method::Get, "/") => {
                let json = fs::read_to_string(&path)?;
                Response::from_string(json)
            }
            (&Method::Post, "/receive") => {
                let mut body = String::new();
                request.as_reader().read_to_string(&mut body)?;
                fs::write(&path, &body)?;          // overwrite local config
                println!("Config updated 👍");
                Response::from_string("ok")
            }
            _ => Response::from_string("404").with_status_code(404),
        };
        let _ = request.respond(resp);
    }
    Ok(())
}
