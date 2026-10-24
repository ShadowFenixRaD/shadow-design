//! Story: Badge (feature `badge`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/badges.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "badges",
        "Badge",
        Category::Dati,
        &["badge"],
        IconKind::CheckCircle,
        "Etichetta di stato compatta, anche a pillola o con punto.",
    )
}
