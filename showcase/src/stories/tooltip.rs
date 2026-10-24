//! Story: Tooltip (feature `tooltip`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/tooltip.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "tooltip",
        "Tooltip",
        Category::Overlay,
        &["tooltip"],
        IconKind::Info,
        "Etichetta flottante al passaggio del puntatore, in quattro direzioni.",
    )
}
