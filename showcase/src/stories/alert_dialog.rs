//! Story: AlertDialog (feature `alert_dialog`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/alert_dialog.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "alert-dialog",
        "AlertDialog",
        Category::Overlay,
        &["alert_dialog"],
        IconKind::AlertTriangle,
        "Dialogo bloccante per conferme critiche e distruttive.",
    )
}
