use dioxus::prelude::*;

#[cfg(feature = "icon")]
use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};

/// Struttura dati per un singolo nodo della gerarchia ad albero.
#[derive(Clone, PartialEq, Debug)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub sublabel: Option<String>,
    pub badge: Option<String>,
    #[cfg(feature = "icon")]
    pub icon: Option<SvgIcon>,
    pub children: Vec<TreeNode>,
    pub default_expanded: bool,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            sublabel: None,
            badge: None,
            #[cfg(feature = "icon")]
            icon: None,
            children: Vec::new(),
            default_expanded: true,
        }
    }

    #[must_use]
    pub fn with_sublabel(mut self, sublabel: impl Into<String>) -> Self {
        self.sublabel = Some(sublabel.into());
        self
    }

    #[must_use]
    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    #[cfg(feature = "icon")]
    #[must_use]
    pub fn with_icon(mut self, icon: SvgIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }

    #[must_use]
    pub fn with_expanded(mut self, expanded: bool) -> Self {
        self.default_expanded = expanded;
        self
    }
}

/// Componente ricorsivo per il rendering di un nodo dell'albero.
#[component]
fn TreeNodeRow(
    node: TreeNode,
    level: usize,
    selected_id: Option<Signal<Option<String>>>,
    on_select: Option<EventHandler<String>>,
) -> Element {
    let mut is_expanded = use_signal(|| node.default_expanded);
    let has_children = !node.children.is_empty();

    let node_id = node.id.clone();
    let is_selected = selected_id
        .map(|sig| sig().as_deref() == Some(&node_id))
        .unwrap_or(false);

    let selected_class = if is_selected { "sd-tree-node--selected" } else { "" };
    let indent_px = level * 18;

    let aria_expanded = if has_children {
        Some(if is_expanded() { "true" } else { "false" })
    } else {
        None
    };

    rsx! {
        div {
            class: "sd-tree-node-wrapper",
            role: "treeitem",
            aria_expanded: aria_expanded,
            aria_selected: if is_selected { "true" } else { "false" },

                div {
                    class: "sd-tree-node {selected_class}",
                    style: "padding-left: {indent_px}px;",
                onclick: {
                    let id_click = node.id.clone();
                    move |_| {
                        if let Some(mut sig) = selected_id {
                            *sig.write() = Some(id_click.clone());
                        }
                        if let Some(handler) = &on_select {
                            handler.call(id_click.clone());
                        }
                    }
                },

                // Pulsante chevron per espansione se ha figli
                if has_children {
                    button {
                        r#type: "button",
                        class: "sd-tree-node__toggle",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            let current = is_expanded();
                            *is_expanded.write() = !current;
                        },
                        {
                            let chevron_class = if is_expanded() { "sd-tree-node__chevron sd-tree-node__chevron--open" } else { "sd-tree-node__chevron" };
                            rsx! {
                                Icon {
                                    kind: IconKind::ChevronRight,
                                    size: 14,
                                    class: "{chevron_class}",
                                }
                            }
                        }
                    }
                } else {
                    span { class: "sd-tree-node__toggle-spacer" }
                }

                // Icona del nodo
                if let Some(custom_icon) = &node.icon {
                    RenderSvgIcon {
                        icon: custom_icon.clone(),
                        size: 16,
                        class: "sd-tree-node__icon".to_string(),
                    }
                }

                // Testo principale e sublabel
                div {
                    class: "sd-tree-node__content",
                    span { class: "sd-tree-node__label", "{node.label}" }
                    if let Some(sub) = &node.sublabel {
                        span { class: "sd-tree-node__sublabel", "{sub}" }
                    }
                }

                // Badge opzionale
                if let Some(b) = &node.badge {
                    span { class: "sd-tree-node__badge", "{b}" }
                }
            }

            // Figli ricorsivi se espanso
            if has_children && is_expanded() {
                div {
                    class: "sd-tree-node__children",
                    for child in node.children {
                        TreeNodeRow {
                            key: "{child.id}",
                            node: child,
                            level: level + 1,
                            selected_id: selected_id,
                            on_select: on_select,
                        }
                    }
                }
            }
        }
    }
}

/// Componente TreeView per rappresentare topologie di rete mesh, alberi di directory e gerarchie complesse.
#[component]
pub fn TreeView(
    nodes: Vec<TreeNode>,
    #[props(default)] selected_id: Option<Signal<Option<String>>>,
    #[props(default)] on_select: Option<EventHandler<String>>,
    #[props(default)] class: String,
) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tree_view").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tree_view", &class);
    #[cfg(feature = "theme")]
    let radius = theme_val.radius_scale.sm();

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius = "";

    rsx! {
        div {
            class: "sd-tree-view {radius} {base_override} {class_modifiers} {class}",
            role: "tree",

            for node in nodes {
                TreeNodeRow {
                    key: "{node.id}",
                    node: node,
                    level: 0,
                    selected_id: selected_id,
                    on_select: on_select,
                }
            }
        }
    }
}
