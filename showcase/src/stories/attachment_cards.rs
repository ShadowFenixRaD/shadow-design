//! Story: AttachmentCard (feature `attachment_card`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/attachment_cards.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "attachment-cards",
        "AttachmentCard",
        Category::Contenuti,
        &["attachment_card"],
        IconKind::Paperclip,
        "Scheda per file allegati, documenti e contatti.",
    )
}
