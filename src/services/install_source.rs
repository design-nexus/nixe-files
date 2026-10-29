// SPDX-License-Identifier: MIT

use std::{
    ffi::OsStr,
    io,
    os::unix::ffi::OsStrExt as _,
    path::{Path, PathBuf},
};

pub(crate) fn installed_executable() -> io::Result<PathBuf> {
    std::env::current_exe().map(|path| replaced_executable_path(&path))
}

// Linux appends " (deleted)" to /proc/self/exe after the installed binary is replaced.
fn replaced_executable_path(executable: &Path) -> PathBuf {
    if !executable.exists()
        && let Some(original) = executable
            .as_os_str()
            .as_bytes()
            .strip_suffix(b" (deleted)")
            .map(|original| Path::new(OsStr::from_bytes(original)))
        && original.is_file()
    {
        return original.to_path_buf();
    }
    executable.to_path_buf()
}

#[cfg(test)]
mod tests;
