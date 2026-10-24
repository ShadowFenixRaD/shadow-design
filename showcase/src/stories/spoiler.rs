//! Story: Spoiler (feature `spoiler`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/spoiler.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "spoiler",
        "Spoiler",
        Category::Layout,
        &["spoiler"],
        IconKind::ChevronDown,
        "Contenuto lungo troncato con pulsante mostra di più.",
    )
}
