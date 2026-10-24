//! Story: ContextMenu (feature `context_menu`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/context_menu.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "context-menu",
        "ContextMenu",
        Category::Overlay,
        &["context_menu"],
        IconKind::Terminal,
        "Menu richiamato con il tasto destro, con scorciatoie e checkbox.",
    )
}
