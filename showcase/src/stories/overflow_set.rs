//! Story: OverflowSet (feature `overflow_set`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/overflow_set.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "overflow-set",
        "OverflowSet",
        Category::Navigazione,
        &["overflow_set"],
        IconKind::Layers,
        "Barra di azioni che sposta le voci in eccesso in un menu.",
    )
}
