//! Story: AppShell (feature `app_shell`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/app_shell.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "app-shell",
        "AppShell",
        Category::Layout,
        &["app_shell"],
        IconKind::Box,
        "Layout applicativo a cinque zone: header, navbar, main, aside e footer.",
    )
}
