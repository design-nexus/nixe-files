// SPDX-License-Identifier: MIT

use gtk::prelude::*;

use crate::ui::{
    preferences::PreferenceManager,
    shortcut_reference::{ReferenceScope, reference_sections},
};

use super::{control_row, page_content, scrollable_page, search, settings_group};

/// Every shortcut for the current browser mode, grouped as in the shortcuts dialog.
pub(super) fn keyboard_page() -> gtk::Widget {
    let content = page_content();
    search::tag(&content, "Keyboard");
    let scope = ReferenceScope::window(PreferenceManager::shared().browser_mode());
    for section in reference_sections(scope) {
        let group = settings_group(&content, &crate::i18n::tr(section.title).to_uppercase());
        for shortcut in &section.rows {
            let keys = gtk::Label::new(Some(&shortcut.key_text()));
            keys.add_css_class("settings-shortcut-keys");
            keys.set_valign(gtk::Align::Center);
            group.append(&control_row(&shortcut.action_text(), "", &keys));
        }
    }
    scrollable_page(&content, None)
}
