//! Story: TransferList (feature `transfer_list`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/transfer_list.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "transfer-list",
        "TransferList",
        Category::Dati,
        &["transfer_list"],
        IconKind::Layers,
        "Due colonne per spostare e assegnare elementi, con ricerca.",
    )
}
