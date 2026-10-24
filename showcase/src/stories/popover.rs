//! Story: Popover (feature `popover`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/popover.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "popover",
        "Popover",
        Category::Overlay,
        &["popover"],
        IconKind::Layers,
        "Pannello flottante ancorato con freccia e chiusura esterna.",
    )
}
