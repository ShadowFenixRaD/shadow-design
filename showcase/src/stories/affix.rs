//! Story: Affix (feature `affix`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/affix.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "affix",
        "Affix",
        Category::Overlay,
        &["affix"],
        IconKind::ChevronUp,
        "Elemento fisso che compare dopo uno scroll, come il ritorno in cima.",
    )
}
