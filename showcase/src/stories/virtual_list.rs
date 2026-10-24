//! Story: VirtualList (feature `virtual_list`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/virtual_list.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "virtual-list",
        "VirtualList",
        Category::Dati,
        &["virtual_list"],
        IconKind::Layers,
        "Lista virtualizzata per decine di migliaia di righe.",
    )
}
