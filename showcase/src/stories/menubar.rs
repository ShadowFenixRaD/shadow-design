//! Story: Menubar (feature `menubar`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/menubar.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "menubar",
        "Menubar",
        Category::Navigazione,
        &["menubar"],
        IconKind::Menu,
        "Barra dei menu in stile desktop con scorciatoie.",
    )
}
