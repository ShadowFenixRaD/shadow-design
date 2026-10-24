//! Story: LoadingOverlay (feature `loading_overlay`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/loading_overlay.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "loading-overlay",
        "LoadingOverlay",
        Category::Dati,
        &["loading_overlay"],
        IconKind::Spinner,
        "Velo con spinner che blocca un'area durante il caricamento.",
    )
}
