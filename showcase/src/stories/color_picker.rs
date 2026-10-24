//! Story: ColorPicker (feature `color_picker`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/color_picker.rs` following
//! `docs/showcase/BATCH_GUIDE.md`; the reference stories are `buttons.rs` and `select.rs`.
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub(
        "color-picker",
        "ColorPicker",
        Category::Form,
        &["color_picker"],
        IconKind::Palette,
        "Selettore di colore con slider HSL, alpha e campioni.",
    )
}
