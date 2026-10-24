//! Story: Skeleton (feature `skeleton`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/skeletons.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "skeletons",
        "Skeleton",
        Category::Dati,
        &["skeleton"],
        IconKind::Box,
        "Segnaposto di caricamento con la forma del contenuto.",
    )
}
