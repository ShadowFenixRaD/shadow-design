//! Story: Charts (feature `charts`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/charts.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "charts",
        "Charts",
        Category::Dati,
        &["charts"],
        IconKind::Sparkles,
        "Grafici SVG senza dipendenze JavaScript che seguono il tema.",
    )
}
