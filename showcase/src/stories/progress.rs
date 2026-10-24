//! Story: ProgressBar (feature `progress`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/progress.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "progress",
        "ProgressBar",
        Category::Dati,
        &["progress"],
        IconKind::Spinner,
        "Barra di avanzamento determinata o indeterminata.",
    )
}
