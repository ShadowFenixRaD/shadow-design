//! Static registry built once from `stories::ALL`. Order = sidebar order (ADR-0006 §2).
use std::sync::LazyLock;

use super::schema::{Category, Story};

static STORIES: LazyLock<Vec<Story>> = LazyLock::new(|| crate::stories::ALL.iter().map(|f| f()).collect());

pub fn all() -> &'static [Story] {
    &STORIES
}

pub fn find(slug: &str) -> Option<&'static Story> {
    STORIES.iter().find(|s| s.slug == slug)
}

pub fn by_category(category: Category) -> Vec<&'static Story> {
    STORIES.iter().filter(|s| s.category == category).collect()
}

pub fn neighbours(slug: &str) -> (Option<&'static Story>, Option<&'static Story>) {
    let Some(i) = STORIES.iter().position(|s| s.slug == slug) else { return (None, None) };
    (i.checked_sub(1).and_then(|j| STORIES.get(j)), STORIES.get(i + 1))
}

/// Distinct Cargo features referenced by stories (for the home counters).
pub fn feature_count() -> usize {
    let mut set = std::collections::BTreeSet::new();
    for s in STORIES.iter() {
        for f in s.features {
            set.insert(*f);
        }
    }
    set.len()
}

/// Case-insensitive match on name, slug, tagline, keywords and features (sidebar filter, search).
pub fn matches(story: &Story, query: &str) -> bool {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return true;
    }
    story.name.to_lowercase().contains(&q)
        || story.slug.contains(&q)
        || story.tagline.to_lowercase().contains(&q)
        || story.keywords.iter().any(|k| k.contains(&q))
        || story.features.iter().any(|f| f.contains(&q))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::schema::Status;

    const CRATE_MANIFEST: &str = include_str!("../../../crate/Cargo.toml");

    fn feature_exists(f: &str) -> bool {
        CRATE_MANIFEST
            .lines()
            .any(|l| l.trim_start().starts_with(&format!("{f} =")) || l.trim_start().starts_with(&format!("{f}=")))
    }

    #[test]
    fn registry_has_every_story() {
        assert_eq!(all().len(), 85, "stories/mod.rs must list all 85 stories");
    }

    #[test]
    fn slugs_are_unique_and_kebab() {
        let mut seen = std::collections::BTreeSet::new();
        for s in all() {
            assert!(seen.insert(s.slug), "duplicate slug {}", s.slug);
            assert!(
                s.slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "slug {} is not kebab-case",
                s.slug
            );
        }
    }

    #[test]
    fn slugs_do_not_shadow_static_routes() {
        for s in all() {
            assert!(
                !["theme-builder", "tokens", "typography", "icons"].contains(&s.slug),
                "{} collides with a static page route",
                s.slug
            );
        }
    }

    #[test]
    fn related_slugs_resolve() {
        for s in all() {
            for r in s.related {
                assert!(find(r).is_some(), "{}: related slug {r} does not exist", s.slug);
                assert_ne!(*r, s.slug, "{}: relates to itself", s.slug);
            }
        }
    }

    #[test]
    fn features_exist_in_crate_manifest() {
        for s in all() {
            assert!(!s.features.is_empty(), "{}: needs at least one Cargo feature", s.slug);
            for f in s.features {
                assert!(feature_exists(f), "{}: feature `{f}` is not in crate/Cargo.toml", s.slug);
            }
        }
    }

    #[test]
    fn non_stub_stories_are_complete() {
        for s in all().iter().filter(|s| s.status != Status::Stub) {
            assert!(s.playground.is_some() || s.examples.len() >= 2, "{}: needs a playground or two examples", s.slug);
            assert!(!s.props.is_empty(), "{}: needs at least one PropGroup", s.slug);
            assert!(s.a11y.len() >= 3, "{}: needs at least three accessibility notes", s.slug);
            assert!(!s.related.is_empty(), "{}: needs related stories", s.slug);
            assert!(s.keywords.len() >= 4, "{}: needs at least four search keywords", s.slug);
            let mut ids = std::collections::BTreeSet::new();
            for e in &s.examples {
                assert!(ids.insert(e.id), "{}: duplicate example id {}", s.slug, e.id);
                assert!(!e.code.trim().is_empty(), "{}: example {} has empty code", s.slug, e.id);
                assert!(
                    !e.code.contains("\"sc-") && !e.code.contains(" sc-"),
                    "{}: example {} leaks a showcase class (sc-*) into its code",
                    s.slug,
                    e.id
                );
            }
        }
    }

    #[test]
    fn no_stub_remains() {
        // Phases 1-2 run with SHOWCASE_ALLOW_STUBS=1; the phase-3 verifier runs without it.
        let stubs: Vec<_> = all().iter().filter(|s| s.status == Status::Stub).map(|s| s.slug).collect();
        if std::env::var("SHOWCASE_ALLOW_STUBS").is_err() {
            assert!(stubs.is_empty(), "stub stories left: {stubs:?}");
        }
    }
}
