//! Story: DocumentCard (feature `document_card`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/document_card.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "document-card",
        "DocumentCard",
        Category::Contenuti,
        &["document_card"],
        IconKind::Paperclip,
        "Scheda documento con anteprima, metadati e azioni.",
    )
}
