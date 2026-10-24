//! Story: Indicator (feature `indicator`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/indicator.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "indicator",
        "Indicator",
        Category::Dati,
        &["indicator"],
        IconKind::CheckCircle,
        "Punto di stato o contatore posizionato sull'angolo di un elemento.",
    )
}
