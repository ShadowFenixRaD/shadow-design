//! Story: NavLink (feature `nav_link`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/nav_link.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "nav-link",
        "NavLink",
        Category::Navigazione,
        &["nav_link"],
        IconKind::Menu,
        "Voce di navigazione laterale con stato attivo e sottomenu.",
    )
}
