//! Story: MessageBubble (feature `chat`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/chat.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "chat",
        "MessageBubble",
        Category::Contenuti,
        &["chat"],
        IconKind::MessageSquare,
        "Bolle di messaggio per conversazioni con stato di consegna.",
    )
}
