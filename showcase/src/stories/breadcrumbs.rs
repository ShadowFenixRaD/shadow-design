//! Story: Breadcrumbs (feature `breadcrumbs`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/breadcrumbs.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "breadcrumbs",
        "Breadcrumbs",
        Category::Navigazione,
        &["breadcrumbs"],
        IconKind::ChevronRight,
        "Percorso gerarchico della pagina con ellissi e separatori.",
    )
}
