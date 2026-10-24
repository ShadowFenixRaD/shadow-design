//! Story: Chip (feature `chip`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/chip.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "chip",
        "Chip",
        Category::Form,
        &["chip"],
        IconKind::Check,
        "Capsule selezionabili per filtri e tag, singole o multiple.",
    )
}
