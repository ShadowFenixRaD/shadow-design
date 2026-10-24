//! Story: DataTable (feature `table`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/tables.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "tables",
        "DataTable",
        Category::Dati,
        &["table"],
        IconKind::Layers,
        "Tabelle semantiche e DataTable con ordinamento e paginazione.",
    )
}
