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
    system.append(&accent_row());

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
    dropdown.add_css_class("form-control");
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


/// Files' accent among the suite's swatches. The theme manager's suite poll
/// repaints the window within a second, so this only stores the choice.
fn accent_row() -> gtk::Box {
    const APP: &str = "files";
    let strip = gtk::Box::new(gtk::Orientation::Horizontal, 6);
    strip.set_valign(gtk::Align::Center);
    let mut names: Vec<Option<String>> = vec![None];
    names.extend(nyx_theme::suite_theme(APP).look.swatches.into_iter().map(|(name, _)| Some(name)));
    let buttons: Rc<Vec<(gtk::Button, gtk::CssProvider, Option<String>)>> = Rc::new(
        names
            .into_iter()
            .map(|name| {
                let button = gtk::Button::new();
                button.add_css_class("accent-swatch");
                let tip = match &name {
                    None => crate::i18n::tr("Theme default"),
                    Some(name) => {
                        let mut chars = name.chars();
                        chars
                            .next()
                            .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
                            .unwrap_or_default()
                    }
                };
                if name.is_none() {
                    button.add_css_class("theme-default");
                }
                button.set_tooltip_text(Some(&tip));
                crate::ui::accessibility::set_label(&button, &tip);
                let provider = gtk::CssProvider::new();
                #[expect(deprecated, reason = "GTK has no per-widget CSS provider replacement")]
                button
                    .style_context()
                    .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER + 8);
                strip.append(&button);
                (button, provider, name)
            })
            .collect(),
    );
    let refresh = {
        let buttons = buttons.clone();
        let last = std::cell::RefCell::new(None);
        move || {
            let theme = nyx_theme::suite_theme(APP);
            let swatches = theme.look.swatches;
            let current = nyx_theme::app_swatch(APP);
            let default = theme.look.accents.get(APP).cloned().unwrap_or(theme.palette.accent);
            let state = (swatches.clone(), current.clone(), default.clone());
            if last.borrow().as_ref() == Some(&state) { return; }
            *last.borrow_mut() = Some(state);
            for (button, provider, name) in buttons.iter() {
                let colour = match name {
                    None => Some(default.clone()),
                    Some(name) => swatches.iter().find(|(s, _)| s == name).map(|(_, c)| c.clone()),
                };
                if let Some(colour) = colour {
                    provider.load_from_string(&format!("button.accent-swatch {{ background-color: {colour}; background-image: none; }}"));
                }
                if *name == current {
                    button.add_css_class("selected");
                } else {
                    button.remove_css_class("selected");
                }
            }
        }
    };
    refresh();
    let refresh = Rc::new(refresh);
    for (button, _, name) in buttons.iter() {
        let name = name.clone();
        let refresh = refresh.clone();
        button.connect_clicked(move |_| {
            nyx_theme::set_app_swatch(APP, name.as_deref());
            refresh();
        });
    }
    let row = super::control_row("Accent", "This app's colour for selection and highlights.", &strip);
    row.add_css_class("settings-accent-row");
    row.set_orientation(gtk::Orientation::Vertical);
    row.set_spacing(10);
    strip.set_halign(gtk::Align::Start);
    let weak = row.downgrade();
    gtk::glib::timeout_add_seconds_local(1, move || {
        if weak.upgrade().is_none() {
            return gtk::glib::ControlFlow::Break;
        }
        refresh();
        gtk::glib::ControlFlow::Continue
    });
    row
}
