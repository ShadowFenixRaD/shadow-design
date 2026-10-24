//! Story: Drawer (feature `drawer`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/drawer.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "drawer",
        "Drawer",
        Category::Overlay,
        &["drawer"],
        IconKind::ChevronRight,
        "Pannello laterale a comparsa da sinistra o destra.",
    )
}
