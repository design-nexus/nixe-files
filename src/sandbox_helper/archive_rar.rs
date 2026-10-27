// SPDX-License-Identifier: MIT
//! Adapter to the separately licensed decoder already bound inside the sandbox.
use std::{io::{Read, Write}, path::Path, process::{Command, Stdio}};
const HELPER: &str = "/app/nyx-files-rar";
pub(super) fn run(path: &Path, password: Option<&str>, _writer: &mut impl Write) -> Result<(), String> {
    let mut command = Command::new(HELPER);
    command.arg("extract-rar").arg(path).stdout(Stdio::inherit()).stderr(Stdio::null());
    if password.is_some() { command.arg("--password-stdin").stdin(Stdio::piped()); } else { command.stdin(Stdio::null()); }
    let mut child = command.spawn().map_err(|_| "RAR decoder is unavailable. Reinstall Files.".to_string())?;
    if let Some(password) = password { child.stdin.take().ok_or("RAR password pipe unavailable")?.write_all(password.as_bytes()).map_err(|e| e.to_string())?; }
    if child.wait().map_err(|e| e.to_string())?.success() { Ok(()) } else { Err("RAR decoder could not complete extraction".into()) }
}
pub(super) fn cover_image(path: &Path) -> Result<Vec<u8>, String> {
    let mut child = Command::new(HELPER).arg("cover-rar").arg(path).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().map_err(|_| "RAR decoder is unavailable. Reinstall Files.".to_string())?;
    let mut bytes = Vec::new();
    child.stdout.take().ok_or("RAR output unavailable")?.take(8 * 1024 * 1024 + 1).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    if bytes.len() > 8 * 1024 * 1024 { let _ = child.kill(); let _ = child.wait(); return Err("RAR cover exceeds size limit".into()); }
    if child.wait().map_err(|e| e.to_string())?.success() { Ok(bytes) } else { Err("RAR cover is unavailable".into()) }
}
