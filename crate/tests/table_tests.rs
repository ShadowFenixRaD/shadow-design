#![cfg(feature = "table")]

use dioxus::prelude::*;
use shadow_design::prelude::*;

#[test]
fn test_table_align_classes() {
    assert_eq!(TableAlign::Left.scss_class(), "sd-table--align-left");
    assert_eq!(TableAlign::Center.scss_class(), "sd-table--align-center");
    assert_eq!(TableAlign::Right.scss_class(), "sd-table--align-right");
}

#[test]
fn test_sort_direction_classes_and_transitions() {
    assert_eq!(SortDirection::None.scss_class(), "sd-table-cell--sort-none");
    assert_eq!(SortDirection::Ascending.scss_class(), "sd-table-cell--sort-asc");
    assert_eq!(SortDirection::Descending.scss_class(), "sd-table-cell--sort-desc");

    assert_eq!(SortDirection::None.next(), SortDirection::Ascending);
    assert_eq!(SortDirection::Ascending.next(), SortDirection::Descending);
    assert_eq!(SortDirection::Descending.next(), SortDirection::None);
}

#[test]
fn test_data_table_column_builders() {
    #[derive(Clone, PartialEq)]
    struct PeerNode {
        id: String,
        ip: String,
    }

    let col = DataTableColumn::<PeerNode>::text("id", "Node ID", |n| n.id.clone())
        .with_sortable(true)
        .with_align(TableAlign::Center)
        .with_width("180px");

    assert_eq!(col.key, "id");
    assert_eq!(col.header, "Node ID");
    assert!(col.sortable);
    assert_eq!(col.align, TableAlign::Center);
    assert_eq!(col.width.as_deref(), Some("180px"));
}

#[test]
fn test_table_defaults() {
    assert_eq!(TableAlign::default(), TableAlign::Left);
    assert_eq!(SortDirection::default(), SortDirection::None);
}

#[test]
fn test_data_table_column_comparators() {
    #[derive(Clone, PartialEq, Debug)]
    struct Item {
        name: &'static str,
        score: u32,
    }

    let a = Item { name: "Beta", score: 40 };
    let b = Item { name: "Alpha", score: 90 };

    // text() generates an automatic string comparator
    let col_text = DataTableColumn::<Item>::text("name", "Name", |i| i.name.to_string());
    assert!(col_text.comparator.is_some());
    let cmp_text = col_text.comparator.as_ref().unwrap();
    assert_eq!(cmp_text(&a, &b), std::cmp::Ordering::Greater);
    assert_eq!(cmp_text(&b, &a), std::cmp::Ordering::Less);

    // with_sort_by creates custom comparator
    let col_score = DataTableColumn::<Item>::new("score", "Score", |_| dioxus::prelude::rsx! {})
        .with_sort_by(|i: &Item| i.score);
    assert!(col_score.comparator.is_some());
    let cmp_score = col_score.comparator.as_ref().unwrap();
    assert_eq!(cmp_score(&a, &b), std::cmp::Ordering::Less);
    assert_eq!(cmp_score(&b, &a), std::cmp::Ordering::Greater);
}

