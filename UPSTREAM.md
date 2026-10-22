Derived from https://github.com/lgse/strata at 809e2f5244b24326a73514b071fe8f4cb530c6c1.
MIT license and third-party attribution are retained. Independently maintained by design-nexus.

Licensing: upstream has an MIT top-level license and GPL-3.0-or-later source files (including the updater and browser animation/completion modules). The combined Files application is distributed under GPL-3.0-or-later; original MIT and other per-file notices remain intact. LICENSE and COPYING contain GPLv3; LICENSE-MIT preserves the original MIT notice. Source for each release is available at its corresponding repository tag.

UnRAR is isolated in the standalone MIT/UnRAR-licensed `rar-helper` program. Its native decoder and cover selection reuse upstream MIT code. The GPL Files executable communicates with it through the existing bounded stream protocol inside the preview sandbox; it does not link the restricted native library. The installer and signed archives include both programs and their notices.
