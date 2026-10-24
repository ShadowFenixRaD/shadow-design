//! Story: CopyButton (feature `copy_button`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/copy_button.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "copy-button",
        "CopyButton",
        Category::Utility,
        &["copy_button"],
        IconKind::Copy,
        "Copia un valore negli appunti con conferma visiva.",
    )
}
