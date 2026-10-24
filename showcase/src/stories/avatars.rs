//! Story: Avatar (feature `avatar`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/avatars.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "avatars",
        "Avatar",
        Category::Contenuti,
        &["avatar"],
        IconKind::Sparkles,
        "Immagine o iniziali dell'utente con stato di presenza.",
    )
}
