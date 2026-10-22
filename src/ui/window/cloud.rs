// SPDX-License-Identifier: MIT
//! Cloud accounts use the existing local navigation/operation stack through FUSE.
use super::*;
use nixe_cloud::{Client, Remote};

pub(super) fn append(state: &Rc<SidebarState>) {
    if state.local_only {
        return;
    }
    state.append_separator();
    state.append_heading("CLOUD STORAGE");
    let holder = gtk::Box::new(gtk::Orientation::Vertical, 0);
    state.widget.append(&holder);
    refresh(&holder, state);
    let weak = holder.downgrade();
    let weak_state = Rc::downgrade(state);
    glib::spawn_future_local(async move {
        let Ok(bus) = gio::bus_get_future(gio::BusType::Session).await else {
            return;
        };
        let callback_holder = weak.clone();
        let callback_state = weak_state.clone();
        let subscription = bus.subscribe_to_signal(
            Some(nixe_cloud::BUS),
            Some(nixe_cloud::BUS),
            Some("Changed"),
            Some(nixe_cloud::PATH),
            None,
            gio::DBusSignalFlags::NONE,
            move |_| {
                if let (Some(holder), Some(state)) =
                    (callback_holder.upgrade(), callback_state.upgrade())
                {
                    refresh(&holder, &state);
                }
            },
        );
        if let Some(holder) = weak.upgrade() {
            let subscription = RefCell::new(Some(subscription));
            holder.connect_destroy(move |_| {
                subscription.borrow_mut().take();
            });
        }
    });
}
fn refresh(holder: &gtk::Box, state: &Rc<SidebarState>) {
    let weak = holder.downgrade();
    let weak_state = Rc::downgrade(state);
    glib::spawn_future_local(async move {
        let result = gio::spawn_blocking(|| Client::connect()?.list()).await;
        let (Some(holder), Some(state)) = (weak.upgrade(), weak_state.upgrade()) else {
            return;
        };
        while let Some(child) = holder.first_child() {
            holder.remove(&child);
        }
        match result {
            Ok(Ok(remotes)) => {
                for remote in remotes {
                    append_remote(&holder, &state, remote);
                }
            }
            _ => {
                let label = gtk::Label::new(Some(&crate::i18n::tr("Cloud Storage unavailable")));
                label.add_css_class("dim-label");
                label.set_wrap(true);
                holder.append(&label);
            }
        }
        let manage = sidebar_button(
            crate::assets::icons::CLOUD,
            &crate::i18n::tr("Manage cloud accounts"),
        );
        manage.connect_clicked(|_| {
            let _ = gio::Subprocess::newv(
                &[
                    std::ffi::OsStr::new("nixe-settings"),
                    std::ffi::OsStr::new("--section"),
                    std::ffi::OsStr::new("cloud-storage"),
                ],
                gio::SubprocessFlags::NONE,
            );
        });
        holder.append(&manage);
    });
}
fn append_remote(holder: &gtk::Box, state: &Rc<SidebarState>, remote: Remote) {
    let label = format!(
        "{} · {}",
        remote.name,
        crate::i18n::tr(match remote.connection.as_str() {
            "connected" => "Connected",
            "connecting" => "Connecting…",
            "authentication_required" => "Authentication required",
            "error" => "Error",
            _ => "Disconnected",
        })
    );
    let row = sidebar_button(crate::assets::icons::CLOUD, &label);
    let weak = Rc::downgrade(state);
    row.connect_clicked(move |button| {
        let Some(state) = weak.upgrade() else { return; };
        button.set_sensitive(false);
        let weak_button = button.downgrade(); let weak_state = Rc::downgrade(&state);
        let remote = remote.clone();
        let pending = Rc::new(Cell::new(true));
        let cancelled = pending.clone();
        state.browser.set_navigation_cleanup(move || cancelled.set(false));
        super::super::notice::show_status(&state.view.overlay(), &crate::i18n::tr("Connecting…"), Duration::from_secs(35));
        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(move || -> anyhow::Result<String> {
                let client = Client::connect()?;
                let current = client.list()?.into_iter().find(|r| r.id == remote.id).ok_or_else(|| anyhow::anyhow!("Account unavailable"))?;
                if current.mount_status == "mounted" { return Ok(current.mount_path); }
                client.mount(&remote.id)?;
                for _ in 0..175 {
                    std::thread::sleep(Duration::from_millis(200));
                    let r = client.list()?.into_iter().find(|r| r.id == remote.id).ok_or_else(|| anyhow::anyhow!("Account unavailable"))?;
                    if r.mount_status == "mounted" { return Ok(r.mount_path); }
                    if r.connection == "error" || r.connection == "authentication_required" { return Err(anyhow::anyhow!("Connection failed")); }
                }
                Err(anyhow::anyhow!("Connection timed out"))
            }).await;
            if let Some(button) = weak_button.upgrade() { button.set_sensitive(true); }
            let Some(state) = weak_state.upgrade() else { return; };
            if !pending.get() { return; }
            state.browser.finish_navigation_cleanup();
            match result {
                Ok(Ok(path)) => { state.browser.navigate(Location::local(PathBuf::from(path))); state.view.focus_file_view(); },
                _ => {
                    super::super::notice::show_status(&state.view.overlay(), &crate::i18n::tr("Cloud connection failed. Retry the account or open Cloud Storage in Settings."), Duration::from_secs(12));
                }
            }
        });
    });
    holder.append(&row);
}
