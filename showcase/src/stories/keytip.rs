//! Story: Keytip (feature `keytip`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/keytip.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "keytip",
        "Keytip",
        Category::Overlay,
        &["keytip"],
        IconKind::Terminal,
        "Suggerimenti di scorciatoia da tastiera sopra i comandi.",
    )
}
