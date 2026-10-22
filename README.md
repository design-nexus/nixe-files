# Files

Independent Strata-derived Rust/GTK4 file manager for the Nixe desktop.
Upstream file-management functionality and visual design are retained, with
integrated cloud navigation and selectable preview presentation.

- Cloud Storage lists accounts configured or adopted in Settings.
- Selecting an account mounts it through Cloud and opens the filesystem.
- Ordinary file operations use the existing asynchronous operation engine.
- Files continues to manage local files when Cloud is absent.
- Settings → General → Browsing → Preview style selects Popup (default) or Panel.
- Popup previews share one resizable, nonmodal window. Space toggles it, Escape
  closes it, and Up/Down follows files in the owning browser.
- Panel retains the embedded preview. Both use existing sandboxed renderers.

## Build and install

Requires Rust, GTK4 4.12+, GtkSourceView5, Poppler GLib, Fontconfig,
GStreamer with app/base development libraries, and the existing preview helpers
(including bubblewrap for isolated native decoding). Install `nixe-cloud`, rclone,
and FUSE3 for cloud features; Settings provides graphical account configuration.

```
cargo build --release --locked
./install-nixe.sh
```

The installer installs the binary, launcher and icon and makes Files the default
folder handler. An existing per-user FileManager1 activation file is backed up
before replacement. Installation does not launch or restart the application.
Settings/state/cache are independent of Strata. Existing Strata data is preserved.

Public dependencies are pinned to immutable Git revisions. CI builds the app
independently. Tagged x86_64 releases package installation resources and publish
signed update manifests using this repository's dedicated Ed25519 release key.
The private key is never committed; release automation reads UPDATE_SIGNING_KEY.

See [Cloud](https://github.com/design-nexus/nixe-cloud) for mount locations,
configuration security, upload/cache semantics, and service API documentation.
A successful local write can still await cloud upload. Account removal never
deletes cloud data. Quotas, sharing links, offline pinning, and cloud-wide search
are secondary features and are not implemented.

Licensing/provenance: [UPSTREAM.md](UPSTREAM.md), original MIT [LICENSE](LICENSE),
GPLv3 [COPYING](COPYING), and [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
The combined application includes GPL-3.0-or-later source files. RAR/CBR decoding
uses a separate MIT/UnRAR-licensed helper, included by the installer; its native
code is not linked into the GPL executable.
