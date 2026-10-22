// SPDX-License-Identifier: MIT
//! Standalone UnRAR program. No GPL application modules are linked here.
use std::io::{Read, Write};
#[allow(dead_code, reason = "The decoder shares the MIT wire protocol with its reader in Files")]
mod rar_extraction { include!(concat!(env!("OUT_DIR"), "/rar_wire.rs")); }
mod sandbox_helper;
mod adapters {
    pub(crate) const PASSWORD_REQUIRED: &str = "A password is required to extract this archive.";
    pub(crate) const MAYBE_BAD_PASSWORD: &str = "The password may be incorrect.";
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let result = (|| -> Result<(), String> {
        let path = args.get(2).ok_or("Missing archive path")?;
        match args.get(1).map(String::as_str) {
            Some("extract-rar") => {
                let password = if args.get(3).is_some_and(|s| s == "--password-stdin") {
                    let mut bytes = Vec::new();
                    std::io::stdin().take(65537).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
                    if bytes.len() > 65536 { return Err("Archive password is too long".into()); }
                    Some(String::from_utf8(bytes).map_err(|_| "Invalid archive password")?)
                } else { None };
                sandbox_helper::extract(std::path::Path::new(path), password.as_deref())
            },
            Some("cover-rar") => std::io::stdout().write_all(&sandbox_helper::cover(std::path::Path::new(path))?).map_err(|e| e.to_string()),
            _ => Err("Invalid RAR decoder operation".into()),
        }
    })();
    if result.is_err() { std::process::exit(1); }
}
