//! Story: Alert (feature `alert`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/alerts.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "alerts",
        "Alert",
        Category::Dati,
        &["alert"],
        IconKind::AlertTriangle,
        "Avviso in linea per informazioni, successi, avvertenze ed errori.",
    )
}
