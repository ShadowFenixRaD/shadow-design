//! Story: MediaCard (feature `media_card`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/media_card.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "media-card",
        "MediaCard",
        Category::Contenuti,
        &["media_card"],
        IconKind::Box,
        "Scheda multimediale a rapporto fisso con area d'azione.",
    )
}
