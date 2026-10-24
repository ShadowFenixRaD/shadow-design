//! Story: Coachmark (feature `coachmark`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/coachmark.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "coachmark",
        "Coachmark",
        Category::Overlay,
        &["coachmark"],
        IconKind::Sparkles,
        "Bolla di guida passo passo per l'onboarding.",
    )
}
