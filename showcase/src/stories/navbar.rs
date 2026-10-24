//! Story: Navbar (feature `navbar`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/navbar.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "navbar",
        "Navbar",
        Category::Layout,
        &["navbar"],
        IconKind::Menu,
        "Barra di navigazione superiore con marchio, link e menu mobile.",
    )
}
