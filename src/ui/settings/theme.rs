// SPDX-License-Identifier: MIT

use std::rc::Rc;

use gtk::{gio, prelude::*};

use crate::{
    assets::icons,
    ui::{
        preferences::{InterfaceRenderer, PreferenceManager},
        theme::ThemeManager,
    },
};

use super::{
    bindings::{bind_switch, bind_theme_switch},
    page_content, scrollable_page,
};

pub(super) struct ThemePage {
    pub(super) widget: gtk::Widget,
    pub(super) flows: Vec<(gtk::FlowBox, u32)>,
    pub(super) dismiss: Rc<dyn Fn()>,
}

/// Appearance. The theme, its font, shapes and colors are the suite's, shared by
/// every Nyx app; text size is the suite's too, with Files' zoom on top.
pub(super) fn theme_page(
    preferences: Rc<PreferenceManager>,
    themes: Rc<ThemeManager>,
) -> ThemePage {
    let content = page_content();
    content.add_css_class("theme-page");

    let system = super::settings_group(&content, "THEME");
    let follow = append_follow_omarchy_option(&system, &themes);
    system.append(&theme_choice_row(&themes));

    let rendering = super::settings_group(&content, "RENDERING");
    let renderer = super::bindings::choice_menu(
        &preferences,
        "Interface renderer",
        &[
            ("GTK default", InterfaceRenderer::System),
            ("Cairo", InterfaceRenderer::Cairo),
        ],
        PreferenceManager::interface_renderer,
        PreferenceManager::set_interface_renderer,
    );
    let renderer_controls = gtk::Box::new(gtk::Orientation::Vertical, 8);
    renderer_controls.set_hexpand(true);
    renderer.set_hexpand(true);
    renderer.set_halign(gtk::Align::Fill);
    renderer_controls.append(&renderer);
    let restart = gtk::Button::with_label(&crate::i18n::tr("Restart now"));
    restart.set_hexpand(true);
    restart.set_halign(gtk::Align::Fill);
    restart.add_css_class("settings-action-button");
    crate::ui::accessibility::set_label(
        &restart,
        &crate::i18n::tr("Restart to apply interface renderer"),
    );
    preferences.bind_preference(
        &restart,
        PreferenceManager::interface_renderer_restart_required,
        |widget, required| widget.set_visible(required),
    );
    restart.connect_clicked(|_| {
        let application = gio::Application::default().and_downcast::<gtk::Application>();
        super::restart(application.as_ref());
    });
    renderer_controls.append(&restart);
    let renderer_row = super::control_row(
        "Interface renderer",
        "Cairo avoids text artifacts on some displays. Restart to apply changes; GSK_RENDERER overrides this choice.",
        &renderer_controls,
    );
    renderer_row.add_css_class("settings-renderer-row");
    renderer_row.set_orientation(gtk::Orientation::Vertical);
    rendering.append(&renderer_row);

    append_thumbnails_option(&content, &preferences);
    let motion = super::settings_group(&content, "MOTION");
    let (row, toggle) = super::settings_option(
        "Reduce motion",
        "Disable nonessential interface animations.",
        preferences.reduce_motion(),
    );
    bind_switch(
        &preferences,
        &toggle,
        PreferenceManager::reduce_motion,
        PreferenceManager::set_reduce_motion,
    );
    motion.append(&row);

    let scroller = scrollable_page(&content, None);
    bind_theme_switch(
        &themes,
        &follow,
        ThemeManager::follows_omarchy,
        ThemeManager::set_follow_omarchy,
    );
    ThemePage {
        widget: scroller,
        flows: Vec::new(),
        dismiss: Rc::new(|| {}),
    }
}

/// The suite's theme, and a button that opens its file to tune it; saving the
/// file restyles every Nyx app.
fn theme_choice_row(themes: &Rc<ThemeManager>) -> gtk::Box {
    let list = themes.themes();
    let names = gtk::StringList::new(&[]);
    for theme in &list {
        names.append(&theme.tokens.name);
    }
    let ids: Rc<Vec<String>> = Rc::new(list.into_iter().map(|theme| theme.id).collect());
    let dropdown = gtk::DropDown::builder().model(&names).build();
    dropdown.set_valign(gtk::Align::Center);
    crate::ui::accessibility::set_label(&dropdown, &crate::i18n::tr("Theme"));
    let edit = gtk::Button::builder()
        .tooltip_text(crate::i18n::tr("Edit theme file"))
        .valign(gtk::Align::Center)
        .build();
    edit.set_child(Some(&crate::assets::chrome_icon(icons::PENCIL)));
    edit.add_css_class("flat");
    crate::ui::accessibility::set_label(&edit, &crate::i18n::tr("Edit theme file"));
    let controls = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    controls.append(&dropdown);
    controls.append(&edit);
    let row = super::control_row(
        "Theme",
        "The colors every Nyx app uses when not following Omarchy.",
        &controls,
    );

    let syncing = Rc::new(std::cell::Cell::new(false));
    themes.bind_theme_preference(
        &controls,
        |manager| (manager.follows_omarchy(), manager.selected_id()),
        {
            let dropdown = dropdown.clone();
            let ids = ids.clone();
            let syncing = syncing.clone();
            move |_, (following, selected): (bool, String)| {
                dropdown.set_sensitive(!following);
                if let Some(position) = ids.iter().position(|id| *id == selected) {
                    syncing.set(true);
                    dropdown.set_selected(position as u32);
                    syncing.set(false);
                }
            }
        },
    );
    {
        let themes = themes.clone();
        let ids = ids.clone();
        dropdown.connect_selected_notify(move |dropdown| {
            if syncing.get() {
                return;
            }
            if let Some(id) = ids.get(dropdown.selected() as usize) {
                themes.select_theme(id);
            }
        });
    }
    {
        let ids = ids.clone();
        edit.connect_clicked(move |_| {
            let id = ids.get(dropdown.selected() as usize).cloned().unwrap_or_default();
            if let Some(path) = nyx_theme::edit_copy(&id) {
                nyx_core::cmd::open_in_editor(&path);
            }
        });
    }
    row
}

fn append_follow_omarchy_option(content: &gtk::Box, themes: &Rc<ThemeManager>) -> gtk::Switch {
    let (row, follow) = super::settings_option(
        "Follow Omarchy",
        "Use the active Omarchy Quattro theme and switch when the system theme changes.",
        themes.follows_omarchy(),
    );
    content.append(&row);
    themes.bind_theme_preference(
        &row,
        ThemeManager::is_omarchy_available,
        super::search::set_available,
    );
    follow
}

fn append_thumbnails_option(content: &gtk::Box, preferences: &Rc<PreferenceManager>) {
    let group = super::settings_group(content, "FILE ICONS");
    let (row, toggle) = super::settings_option(
        "Show thumbnails",
        "Show thumbnails instead of file-type icons. The preview panel stays available.",
        preferences.show_thumbnails(),
    );
    bind_switch(
        preferences,
        &toggle,
        PreferenceManager::show_thumbnails,
        PreferenceManager::set_show_thumbnails,
    );
    group.append(&row);
    let (row, toggle) = super::settings_option(
        "Colored sidebar icons",
        "Show every sidebar place's icon in its color. Off, only the current place's icon is colored.",
        preferences.colored_sidebar_icons(),
    );
    bind_switch(
        preferences,
        &toggle,
        PreferenceManager::colored_sidebar_icons,
        PreferenceManager::set_colored_sidebar_icons,
    );
    group.append(&row);
}

