// SPDX-License-Identifier: MIT

use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

use super::replaced_executable_path;

#[test]
fn replaced_executable_resolves_to_its_install_path() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let installed = directory.path().join("nyx-files");
    let deleted = directory.path().join("nyx-files (deleted)");
    std::fs::write(&installed, b"").expect("the installed executable to be written");

    assert_eq!(replaced_executable_path(&deleted), installed);
    assert_eq!(replaced_executable_path(&installed), installed);

    let installed = directory.path().join(OsStr::from_bytes(b"nyx-files-\xff"));
    std::fs::write(&installed, b"").expect("a non-UTF-8 executable to be written");
    let deleted = directory
        .path()
        .join(OsStr::from_bytes(b"nyx-files-\xff (deleted)"));
    assert_eq!(replaced_executable_path(&deleted), installed);
}

#[test]
fn deleted_suffix_is_kept_when_it_is_not_a_replaced_executable() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let deleted = directory.path().join("nyx-files (deleted)");

    assert_eq!(replaced_executable_path(&deleted), deleted);

    std::fs::write(&deleted, b"").expect("the suffixed executable to be written");
    std::fs::write(directory.path().join("nyx-files"), b"").expect("a sibling to be written");
    assert_eq!(replaced_executable_path(&deleted), deleted);

    std::fs::remove_file(&deleted).expect("the suffixed executable to be removed");
    std::fs::remove_file(directory.path().join("nyx-files")).expect("the sibling to be removed");
    std::fs::create_dir(directory.path().join("nyx-files")).expect("a directory to be created");
    assert_eq!(replaced_executable_path(&deleted), deleted);
}
