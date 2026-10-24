//! Story: Checkbox (feature `checkbox`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/checkbox.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "checkbox",
        "Checkbox",
        Category::Form,
        &["checkbox"],
        IconKind::Check,
        "Casella di controllo con stato indeterminato, descrizione ed errore.",
    )
}
