// SPDX-License-Identifier: MIT

//! Folders in the home folder that show what they hold: a default decoration
//! for each, used when the folder has no custom icon of its own.

use crate::assets::icons;
use gtk::glib;
use std::path::{Path, PathBuf};

/// Home-folder names, lowercased, and the decoration each one gets.
const HOME_FOLDERS: &[(&str, &str)] = &[
    ("archives", icons::BOX),
    ("documents", icons::DOCUMENTS),
    ("downloads", icons::DOWNLOADS),
    ("dropbox", icons::DROPBOX),
    ("music", icons::MUSIC),
    ("pictures", icons::PICTURES),
    ("porn", icons::EIGHTEEN_PLUS),
    ("projects", icons::GIT_BRANCH),
    ("wallpapers", icons::MONITOR),
    ("wallpaper", icons::MONITOR),
    ("work", icons::BRIEFCASE),
    ("videos", icons::VIDEOS),
    ("games", icons::GAMEPAD),
    ("go", icons::CODE_XML),
];

thread_local! {
    /// The XDG user folders, wherever they're configured.
    static XDG_FOLDERS: Vec<(PathBuf, &'static str)> = [
        (glib::UserDirectory::Documents, icons::DOCUMENTS),
        (glib::UserDirectory::Downloads, icons::DOWNLOADS),
        (glib::UserDirectory::Music, icons::MUSIC),
        (glib::UserDirectory::Pictures, icons::PICTURES),
        (glib::UserDirectory::Videos, icons::VIDEOS),
    ]
    .into_iter()
    .filter_map(|(dir, icon)| Some((glib::user_special_dir(dir)?, icon)))
    .collect();
}

/// The decoration `path` gets when it has no custom icon, if it's one of the
/// special home folders.
pub(crate) fn default_decoration(path: &Path) -> Option<&'static str> {
    let home = crate::ui::window::home_directory();
    XDG_FOLDERS.with(|xdg| decoration_for(path, &home, xdg))
}

fn decoration_for(path: &Path, home: &Path, xdg: &[(PathBuf, &'static str)]) -> Option<&'static str> {
    if let Some((_, icon)) = xdg.iter().find(|(dir, _)| dir != home && dir == path) {
        return Some(icon);
    }
    if path.parent()? != home {
        return None;
    }
    let name = path.file_name()?.to_str()?.to_ascii_lowercase();
    HOME_FOLDERS.iter().find(|(folder, _)| *folder == name).map(|(_, icon)| *icon)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOME: &str = "/home/u";

    fn deco(path: &str, xdg: &[(PathBuf, &'static str)]) -> Option<&'static str> {
        decoration_for(Path::new(path), Path::new(HOME), xdg)
    }

    #[test]
    fn home_folders_get_their_decoration() {
        assert_eq!(deco("/home/u/Work", &[]), Some(icons::BRIEFCASE));
        assert_eq!(deco("/home/u/Porn", &[]), Some(icons::EIGHTEEN_PLUS));
        assert_eq!(deco("/home/u/go", &[]), Some(icons::CODE_XML));
        assert_eq!(deco("/home/u/Wallpapers", &[]), Some(icons::MONITOR));
    }

    #[test]
    fn names_ignore_case() {
        assert_eq!(deco("/home/u/projects", &[]), Some(icons::GIT_BRANCH));
        assert_eq!(deco("/home/u/DROPBOX", &[]), Some(icons::DROPBOX));
    }

    #[test]
    fn only_direct_children_of_home() {
        assert_eq!(deco("/home/u/Work/Documents", &[]), None);
        assert_eq!(deco("/srv/Music", &[]), None);
        assert_eq!(deco("/home/u/Notes", &[]), None);
        assert_eq!(deco(HOME, &[]), None);
    }

    #[test]
    fn xdg_folders_match_wherever_they_are() {
        let xdg = [(PathBuf::from("/data/Photos"), icons::PICTURES)];
        assert_eq!(deco("/data/Photos", &xdg), Some(icons::PICTURES));
        assert_eq!(deco("/data/Other", &xdg), None);
    }

    #[test]
    fn an_xdg_folder_set_to_home_is_ignored() {
        let xdg = [(PathBuf::from(HOME), icons::DOWNLOADS)];
        assert_eq!(deco(HOME, &xdg), None);
    }
}
