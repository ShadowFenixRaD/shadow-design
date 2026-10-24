//! Story: Fieldset (feature `fieldset`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/fieldset.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "fieldset",
        "Fieldset",
        Category::Form,
        &["fieldset"],
        IconKind::Box,
        "Raggruppa i campi correlati di un modulo sotto una legenda.",
    )
}
