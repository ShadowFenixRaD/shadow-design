//! Story: Timeline (feature `timeline`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/timelines.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "timelines",
        "Timeline",
        Category::Dati,
        &["timeline"],
        IconKind::Clock,
        "Cronologia di eventi con stato per ogni passo.",
    )
}
