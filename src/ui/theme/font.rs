// SPDX-License-Identifier: MIT

/// The bundled interface font; the Nerd Font build covers icon glyphs it lacks.
pub(super) const DEFAULT_FAMILIES: [&str; 2] = ["JetBrains Mono", "JetBrainsMono Nerd Font"];

/// The chosen families then `sans-serif`, or the bundled ones then `monospace`
/// when none is chosen. A chosen list names neither the bundled family nor
/// `monospace`: fontconfig rules can put either ahead of everything listed.
fn families(custom: Option<&str>) -> Vec<&str> {
    // A theme may list one family under several names: "Inter, Inter 18pt".
    let chosen: Vec<&str> = custom
        .into_iter()
        .flat_map(|names| names.split(','))
        .map(str::trim)
        .filter(|family| !family.is_empty())
        .collect();
    if chosen.is_empty() {
        DEFAULT_FAMILIES.into_iter().chain(["monospace"]).collect()
    } else {
        chosen.into_iter().chain(["sans-serif"]).collect()
    }
}

/// Interface text rules. GTK's built-in theme resets popover fonts with
/// `font: initial`, so popovers and their children restate the family.
pub(super) fn stylesheet(custom: Option<&str>) -> String {
    let stack = families(custom)
        .into_iter()
        .map(|family| match family {
            "sans-serif" | "monospace" => family.to_owned(),
            _ => css_string(family),
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "*, popover.background, popover.background > contents, popover.background label, popover.background button {{ font-family: {stack}; }}\n"
    )
}

/// The `gtk-font-name` value for widgets that read the setting instead of CSS.
pub(super) fn font_name(custom: Option<&str>, root_font_px: f64) -> String {
    let mut description = gtk::pango::FontDescription::new();
    description.set_family(&families(custom).join(","));
    description.set_absolute_size(root_font_px * f64::from(gtk::pango::SCALE));
    description.to_str().into()
}

fn css_string(value: &str) -> String {
    let mut quoted = String::with_capacity(value.len() + 2);
    quoted.push('"');
    for character in value.chars() {
        match character {
            '"' | '\\' => {
                quoted.push('\\');
                quoted.push(character);
            }
            '\n' | '\r' | '\u{c}' => quoted.push(' '),
            _ => quoted.push(character),
        }
    }
    quoted.push('"');
    quoted
}
