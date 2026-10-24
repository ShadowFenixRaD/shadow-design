# ADR-0005: Three type roles: Kanit, Onest, Source Code Pro

- Status: Accepted
- Date: 2026-10-24

## Context

The library had one sans family (Geist) for everything and Geist Mono for code. Geist is precise but has become a default pick in AI-generated interfaces, so it did not give Shadow Design a voice of its own. The maintainer reviewed about 80 Google Fonts candidates in a side-by-side lab on Lit Glass surfaces, with fonts commonly chosen by AI generators flagged. The maintainer chose a separate face for each role.

## Decision

1. Typography has **three roles**, each a runtime token (`crate/scss/_tokens.scss`, documented in `docs/design/tokens.md`):

   | Token | Family | Use |
   | :-- | :-- | :-- |
   | `--sd-font-heading` + `--sd-weight-heading` (600) | **Kanit** | Page and section headings, the `Title` component, titles of containers and floating layers (card, modal, drawer, alert dialog, popover, coachmark, toast, alert), and large metric values |
   | `--sd-font-body` | **Onest** | Everything else that is read or operated: body text, labels, buttons, items, inputs, table cells, badges |
   | `--sd-font-mono` | **Source Code Pro** | Code, kbd, uppercase section labels, numeric readouts in tables, chart axes |

2. `--sd-font-sans` is removed; `--sd-font-body` replaces it.
3. Headings never use the body face, and controls never use the heading face. A component's title element sets `font-family: var(--sd-font-heading); font-weight: var(--sd-weight-heading)`.
4. All three families load from one Google Fonts import in `_fonts.scss` (Kanit 400–700, Onest 400–700, Source Code Pro 400–600). Self-hosting is still deferred (ADR-0003).

## Consequences

- Swapping a family later is a one-line token change plus the import.
- Kanit has taller vertical metrics than Geist. Titles use `--sd-leading-tight`, and containers must not rely on Geist-specific line boxes.
- Consumers who referenced `--sd-font-sans` must switch to `--sd-font-body` (CHANGELOG).
