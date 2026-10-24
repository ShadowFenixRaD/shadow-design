# Graph Report - shadow-design  (2026-10-24)

## Corpus Check
- 253 files · ~173,869 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 108 file(s) not represented in the graph (top: .scss 94, (none) 7, .toml 2)

## Summary
- 2248 nodes · 3945 edges · 184 communities (96 shown, 88 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 77 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `495dbb9d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- src/main.rs
- IconKind
- avatar.rs
- notification.rs
- ButtonVariant
- AnimationVariant
- ListMarker
- src/rich_text.rs
- JustifyContent
- table.rs
- complex_tooling_tests.rs
- GridSpacing
- SvgIcon
- menubar/mod.rs
- stepper/mod.rs
- app_shell/mod.rs
- prelude
- src/color_picker.rs
- src/overflow_set.rs
- accordion/mod.rs
- BadgeVariant
- FeatureBanner
- ShowcaseTabs
- ControlSize
- src/slider.rs
- use_theme
- IndicatorColor
- ComponentSpacing
- Theme
- timeline.rs
- HeadingOrder
- AGENTS.md — Shadow Design Workspace Guide
- primitives.rs
- PersonaProps
- PopoverPosition
- PropRow
- src/select.rs
- Changelog — Shadow Design
- AlertVariant
- breadcrumbs/mod.rs
- CardShadow
- ChartSeries
- src/chip.rs
- Changelog — Shadow Design
- AGENTS.md — Shadow Design Workspace Guide
- super
- PrimaryColor
- CloseButton
- Signal
- DeliveryStatus
- UseForm<T>
- NavigationLinkItem
- ProgressVariant
- PrimaryColor
- hooks/mod.rs
- area.rs
- src/pin_input.rs
- get_series_color
- ThemeBuilder
- MarqueeSelection
- bottom_navigation/mod.rs
- HoverCardPosition
- SegmentedControl
- src/speed_dial.rs
- pie.rs
- TreeNode
- util.rs
- src/drawer.rs
- modal.rs
- ScrollType
- debounce.rs
- AffixPosition
- src/media_card.rs
- Burger
- CheckboxSize
- src/context_menu.rs
- Locale
- src/number_input.rs
- RadiusScale
- check_classes.py
- 📦 Libreria UI (`crate/`)
- BarChart
- GlowMode
- heatmap.rs
- FieldsetVariant
- nav.rs
- SkeletonShape
- ElevationStyle
- src/transfer_list.rs
- layout.rs
- alerts.rs
- Shadow Design (`shadow-design`)
- ColorFormat
- Orientation
- src/rating.rs
- avatars.rs
- BreadcrumbItemProps
- buttons.rs
- src/tabs.rs
- RatioPreset
- RadarChart
- ContactCard
- src/highlight.rs
- pages/checkbox.rs
- src/center.rs
- UseWindowScroll
- Switch
- Shadow Design — Showcase App
- tables.rs
- pages/coachmark.rs
- click_outside.rs
- accordion.rs
- pages/activity_feed.rs
- pages/affix.rs
- pages/alert_dialog.rs
- app_shell.rs
- pages/aspect_ratio.rs
- attachment_cards.rs
- pages/command_palette.rs
- breadcrumbs.rs
- pages/burger.rs
- pages/carousel.rs
- pages/center.rs
- charts.rs
- pages/chat.rs
- pages/code_highlight.rs
- pages/color_picker.rs
- pages/context_menu.rs
- pages/fieldset.rs
- pages/date_picker.rs
- dividers.rs
- pages/document_card.rs
- pages/drawer.rs
- pages/dropdown.rs
- pages/pin_input.rs
- skeletons.rs
- pages/slider.rs
- pages/group.rs
- pages/highlight.rs
- pages/hover_card.rs
- pages/indicator.rs
- inputs.rs
- pages/keytip.rs
- lists.rs
- pages/marquee_selection.rs
- pages/masonry.rs
- pages/media_card.rs
- modals.rs
- pages/stack.rs
- pages/navbar.rs
- pages/navigation_menu.rs
- pages/number_input.rs
- timelines.rs
- pagination.rs
- reactive_hooks_tests.rs
- pages/progress.rs
- CodeBlock
- pages/rating.rs
- pages/resizable.rs
- pages/rich_text.rs
- scroll_area.rs
- segmented_controls.rs
- pages/speed_dial.rs
- spoiler.rs
- src/coachmark.rs
- pages/switch.rs
- pages/tabs.rs
- pages/textarea.rs
- pages/tooltip.rs
- nav_link/mod.rs
- pages/typography.rs
- AttachmentCard
- STYLES
- shadow-design
- renovate.json

## God Nodes (most connected - your core abstractions)
1. `IconKind` - 134 edges
2. `Route` - 94 edges
3. `PropRow` - 92 edges
4. `ShowcaseTabs()` - 91 edges
5. `FeatureBanner()` - 89 edges
6. `Signal` - 77 edges
7. `Icon()` - 49 edges
8. `Theme` - 48 edges
9. `SvgIcon` - 32 edges
10. `use_theme()` - 27 edges

## Surprising Connections (you probably didn't know these)
- `ReactiveHooksPage()` --calls--> `use_clipboard()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/clipboard.rs
- `Layout()` --calls--> `use_hotkey()`  [INFERRED]
  showcase/src/components/layout.rs → crate/src/hooks/hotkey.rs
- `ReactiveHooksPage()` --calls--> `use_local_storage()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/local_storage.rs
- `ReactiveHooksPage()` --calls--> `use_window_scroll()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/scroll.rs
- `FeatureBanner()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/components/banner.rs → crate/src/theme.rs

## Import Cycles
- 1-file cycle: `crate/src/lib.rs -> crate/src/lib.rs`
- 1-file cycle: `showcase/src/main.rs -> showcase/src/main.rs`
- 2-file cycle: `showcase/src/components/layout.rs -> showcase/src/main.rs -> showcase/src/components/layout.rs`
- 2-file cycle: `crate/src/stepper/mod.rs -> crate/src/stepper/step.rs -> crate/src/stepper/mod.rs`
- 2-file cycle: `crate/src/stepper/completed.rs -> crate/src/stepper/mod.rs -> crate/src/stepper/completed.rs`
- 2-file cycle: `crate/src/menubar/item.rs -> crate/src/menubar/mod.rs -> crate/src/menubar/item.rs`
- 2-file cycle: `crate/src/menubar/menu.rs -> crate/src/menubar/mod.rs -> crate/src/menubar/menu.rs`
- 2-file cycle: `crate/src/app_shell/mod.rs -> crate/src/app_shell/navbar.rs -> crate/src/app_shell/mod.rs`
- 2-file cycle: `crate/src/app_shell/aside.rs -> crate/src/app_shell/mod.rs -> crate/src/app_shell/aside.rs`
- 2-file cycle: `crate/src/accordion/mod.rs -> crate/src/accordion/panel.rs -> crate/src/accordion/mod.rs`
- 3-file cycle: `showcase/src/components/layout.rs -> showcase/src/components/nav.rs -> showcase/src/main.rs -> showcase/src/components/layout.rs`

## Communities (184 total, 88 thin omitted)

### Community 0 - "src/main.rs"
Cohesion: 0.06
Nodes (94): App(), FAVICON, HEADER_SVG, Route, AccordionPage, ActivityFeedPage, AffixPage, AlertDialogPage (+86 more)

### Community 2 - "IconKind"
Cohesion: 0.04
Nodes (55): IconKind, AlertCircle, AlertTriangle, .ALL, Bluetooth, Box, Calendar, Check (+47 more)

### Community 3 - "avatar.rs"
Cohesion: 0.09
Nodes (14): Avatar(), AvatarGroup(), AvatarSize, Lg, Md, Sm, Xl, Xs (+6 more)

### Community 4 - "notification.rs"
Cohesion: 0.07
Nodes (22): NotificationContainer(), NotificationHandle, NotificationItem, NotificationProvider(), NotificationToast(), NotificationVariant, Danger, Info (+14 more)

### Community 5 - "ButtonVariant"
Cohesion: 0.08
Nodes (17): AlertDialog(), ConfirmDialog(), Button(), ButtonSize, Lg, Md, Sm, Xl (+9 more)

### Community 6 - "AnimationVariant"
Cohesion: 0.05
Nodes (37): Animations, .DRAWER_LEFT, .DRAWER_RIGHT, .DROPDOWN_ENTER, .FADE_IN, .FADE_OUT, .INDETERMINATE, .MODAL_ENTER (+29 more)

### Community 7 - "ListMarker"
Cohesion: 0.07
Nodes (32): List(), ListContext, ListItem(), ListMarker, Circle, Custom, Decimal, Disc (+24 more)

### Community 8 - "src/rich_text.rs"
Cohesion: 0.08
Nodes (28): format_inline(), html_escape(), html_to_markdown(), markdown_to_html(), RichTextCommand, Blockquote, Bold, BulletList (+20 more)

### Community 9 - "JustifyContent"
Cohesion: 0.07
Nodes (22): AlignItems, Baseline, Center, End, Start, Stretch, Group(), JustifyContent (+14 more)

### Community 10 - "table.rs"
Cohesion: 0.09
Nodes (19): DataTable(), DataTableColumn, DataTableColumn<T>, SortDirection, Ascending, Descending, None, Table() (+11 more)

### Community 11 - "complex_tooling_tests.rs"
Cohesion: 0.05
Nodes (15): CodeBlock(), CodeHighlight(), SyntaxToken, SyntaxTokenKind, Comment, Function, Keyword, Number (+7 more)

### Community 12 - "GridSpacing"
Cohesion: 0.08
Nodes (22): Grid(), GridAlign, Baseline, Center, End, Start, Stretch, GridCol() (+14 more)

### Community 13 - "SvgIcon"
Cohesion: 0.24
Nodes (6): render_icon_paths(), RenderSvgIcon(), SvgIcon, Custom, Kind, TextInput()

### Community 14 - "menubar/mod.rs"
Cohesion: 0.14
Nodes (16): MenubarItem(), MenubarItemProps, MenubarContent(), MenubarContentProps, MenubarMenu(), MenubarMenuProps, MenubarTrigger(), MenubarTriggerProps (+8 more)

### Community 15 - "stepper/mod.rs"
Cohesion: 0.12
Nodes (16): StepperCompleted(), StepperCompletedProps, Stepper(), StepperCtx, StepperOrientation, .ALL, Horizontal, Vertical (+8 more)

### Community 16 - "app_shell/mod.rs"
Cohesion: 0.14
Nodes (13): AppShellAside(), AppShellAsideProps, AppShellFooter(), AppShellFooterProps, AppShellHeader(), AppShellHeaderProps, AppShellMain(), AppShellMainProps (+5 more)

### Community 17 - "prelude"
Cohesion: 0.09
Nodes (7): Keytip(), Navbar(), copy_text(), BadgesPage(), MenubarPage(), PopoverPage(), RadioPage()

### Community 18 - "src/color_picker.rs"
Cohesion: 0.18
Nodes (21): ColorPicker(), default_color_swatches(), format_color(), hsl_to_rgba(), Hsla, hsv_to_rgb(), parse_color(), parse_hex() (+13 more)

### Community 19 - "src/overflow_set.rs"
Cohesion: 0.11
Nodes (12): Dropdown(), DropdownCtx, DropdownDivider(), DropdownItem(), DropdownPlacement, BottomLeft, BottomRight, TopLeft (+4 more)

### Community 20 - "accordion/mod.rs"
Cohesion: 0.10
Nodes (13): AccordionControl(), AccordionItem(), Accordion(), AccordionContext, AccordionItemContext, AccordionVariant, Contained, Default (+5 more)

### Community 21 - "BadgeVariant"
Cohesion: 0.14
Nodes (10): Badge(), BadgeVariant, Danger, Default, Outline, Primary, Success, Warning (+2 more)

### Community 22 - "FeatureBanner"
Cohesion: 0.08
Nodes (8): FeatureBanner(), BottomNavigationPage(), CardsPage(), ChipPage(), CloseButtonPage(), FacepilePage(), TransferListPage(), TreesPage()

### Community 23 - "ShowcaseTabs"
Cohesion: 0.10
Nodes (7): ShowcaseTabs(), AnimationsPage(), GridPage(), LoadingOverlayPage(), NavLinkPage(), OverflowSetPage(), VirtualListPage()

### Community 24 - "ControlSize"
Cohesion: 0.19
Nodes (11): calculate_pagination_range(), PaginationControl(), PaginationControlProps, PaginationItem, Ellipsis, Page, test_pagination_range_at_start(), test_pagination_range_small_total() (+3 more)

### Community 25 - "src/slider.rs"
Cohesion: 0.14
Nodes (12): calc_percentage(), RangeSlider(), Slider(), SliderMark, snap_to_step(), test_slider_marks(), Tooltip(), TooltipPosition (+4 more)

### Community 26 - "use_theme"
Cohesion: 0.06
Nodes (22): Carousel(), CarouselSlide(), CommandAction, CommandPalette(), Calendar(), CalendarViewMode, Day, Month (+14 more)

### Community 27 - "IndicatorColor"
Cohesion: 0.11
Nodes (16): Indicator(), IndicatorColor, Amber, Cyan, Emerald, Rose, Violet, IndicatorPosition (+8 more)

### Community 28 - "ComponentSpacing"
Cohesion: 0.13
Nodes (11): ComponentSpacing, Lg, Md, None, Sm, Xl, Xs, ImageList() (+3 more)

### Community 30 - "timeline.rs"
Cohesion: 0.14
Nodes (15): Timeline(), TimelineAlign, Left, Right, TimelineBulletState, Active, Completed, Default (+7 more)

### Community 31 - "HeadingOrder"
Cohesion: 0.13
Nodes (17): Code(), Heading(), HeadingOrder, H1, H2, H3, H4, H5 (+9 more)

### Community 32 - "AGENTS.md — Shadow Design Workspace Guide"
Cohesion: 0.08
Nodes (22): 0. Navigazione del Codice con graphify, 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Feature Flags, 4. Sistema di Stile (100% SCSS), 5. Theme Engine (`crate/src/theme.rs`), 6. Showcase (`showcase/`), 7. Regole di Sviluppo (+14 more)

### Community 33 - "primitives.rs"
Cohesion: 0.17
Nodes (16): CurveType, Linear, Monotone, Step, DEFAULT_CHART_COLORS, generate_stacked_area_band_path(), generate_svg_area_path(), generate_svg_path() (+8 more)

### Community 34 - "PersonaProps"
Cohesion: 0.13
Nodes (6): ActivityComment, ActivityEvent, ActivityFeed(), ActivityItem(), PersonaProps, DocumentCard()

### Community 35 - "PopoverPosition"
Cohesion: 0.13
Nodes (17): Popover(), PopoverContext, PopoverDropdown(), PopoverPosition, Bottom, BottomEnd, BottomStart, Left (+9 more)

### Community 36 - "PropRow"
Cohesion: 0.12
Nodes (7): PropRow, PropsTable(), CopyButtonPage(), DropzonePage(), UserRegistrationModel, SelectPage(), StepperPage()

### Community 37 - "src/select.rs"
Cohesion: 0.23
Nodes (11): use_locale(), Combobox(), display_label(), filter_items(), MultiSelect(), Select(), SelectItem, SelectOption() (+3 more)

### Community 38 - "Changelog — Shadow Design"
Cohesion: 0.07
Nodes (29): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, [0.5.0] — 2026-10-24, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti (+21 more)

### Community 39 - "AlertVariant"
Cohesion: 0.16
Nodes (6): Alert(), AlertVariant, Danger, Info, Success, Warning

### Community 40 - "breadcrumbs/mod.rs"
Cohesion: 0.23
Nodes (8): BreadcrumbData, Breadcrumbs(), BreadcrumbsProps, calculate_collapse_bounds(), test_breadcrumb_data_builder(), test_collapse_bounds_custom_overrides(), test_collapse_bounds_not_exceeding_max(), test_collapse_bounds_with_max_3()

### Community 41 - "CardShadow"
Cohesion: 0.12
Nodes (12): Card(), CardShadow, Lg, Md, None, Sm, Xl, CardVariant (+4 more)

### Community 42 - "ChartSeries"
Cohesion: 0.14
Nodes (6): AreaChart(), LineChart(), ChartDataPoint, ChartSeries, compute_min_max(), test_point_builder()

### Community 43 - "src/chip.rs"
Cohesion: 0.15
Nodes (14): ControlSize, Lg, Md, Sm, Chip(), ChipGroup(), ChipGroupContext, ChipGroupProps (+6 more)

### Community 44 - "Changelog — Shadow Design"
Cohesion: 0.07
Nodes (29): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, [0.5.0] — 2026-10-24, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti (+21 more)

### Community 45 - "AGENTS.md — Shadow Design Workspace Guide"
Cohesion: 0.10
Nodes (20): 0. Navigazione del Codice con graphify, 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Feature Flags, 4. Sistema di Stile (100% SCSS), 5. Theme Engine (`crate/src/theme.rs`), 6. Showcase (`showcase/`), 7. Regole di Sviluppo (+12 more)

### Community 47 - "PrimaryColor"
Cohesion: 0.16
Nodes (7): PrimaryColor, Crimson, Cyan, Emerald, Obsidian, Violet, ThemeProvider()

### Community 48 - "CloseButton"
Cohesion: 0.13
Nodes (10): AriaHeading(), CloseButton(), CloseButtonSize, Lg, Md, Sm, CloseButtonVariant, Filled (+2 more)

### Community 49 - "Signal"
Cohesion: 0.18
Nodes (10): Signal, Radio(), RadioCard(), RadioContext, RadioGroup(), RadioOrientation, Horizontal, Vertical (+2 more)

### Community 50 - "DeliveryStatus"
Cohesion: 0.16
Nodes (9): BubbleDirection, Incoming, Outgoing, DeliveryStatus, Delivered, Failed, Sending, Sent (+1 more)

### Community 51 - "UseForm<T>"
Cohesion: 0.15
Nodes (6): FormOptions, FormValidator, use_form(), UseForm, UseForm<T>, FormValidationPage()

### Community 52 - "NavigationLinkItem"
Cohesion: 0.21
Nodes (5): NavigationLinkItem, NavigationMenu(), NavigationMenuItem, NavigationMenuSection, NavigationPreviewCard

### Community 53 - "ProgressVariant"
Cohesion: 0.16
Nodes (10): ProgressBar(), ProgressSize, Lg, Md, Sm, ProgressVariant, Danger, Primary (+2 more)

### Community 54 - "PrimaryColor"
Cohesion: 0.13
Nodes (5): ColorTokens, ComponentOverride, sanitize_css_prefix(), test_cartoon_apply_primary_color_keeps_components(), test_primary_color_custom_with_prefix_sanitized()

### Community 55 - "hooks/mod.rs"
Cohesion: 0.19
Nodes (8): use_debounced_signal(), use_throttle(), use_hotkey(), use_idle(), use_intersection_observer(), use_media_query(), test_debounced_signal_initial_value(), ReactiveHooksPage()

### Community 56 - "area.rs"
Cohesion: 0.33
Nodes (4): ChartContainer(), ChartGrid(), ChartLegend(), ChartTooltip()

### Community 57 - "src/pin_input.rs"
Cohesion: 0.13
Nodes (10): use_unique_id(), ColorInput(), PinInput(), PinInputProps, PinInputSize, PinInputType, Alphanumeric, Numeric (+2 more)

### Community 58 - "get_series_color"
Cohesion: 0.20
Nodes (7): get_series_color(), BubbleChart(), ScaleType, Linear, Log, ScatterChart(), ScatterPoint

### Community 59 - "ThemeBuilder"
Cohesion: 0.18
Nodes (5): Custom, test_builder_preserves_overrides_when_changing_color(), test_primary_color_custom_classes(), ThemeBuilder, is_valid_hex_color()

### Community 61 - "bottom_navigation/mod.rs"
Cohesion: 0.21
Nodes (7): BottomNavigation(), BottomNavigationProps, BottomNavPosition, Absolute, .ALL, Fixed, Relative

### Community 62 - "HoverCardPosition"
Cohesion: 0.22
Nodes (9): HoverCard(), HoverCardContext, HoverCardDropdown(), HoverCardPosition, Bottom, Left, Right, Top (+1 more)

### Community 63 - "SegmentedControl"
Cohesion: 0.19
Nodes (6): SegmentedControl(), SegmentedControlSize, Lg, Md, Sm, SegmentOption

### Community 64 - "src/speed_dial.rs"
Cohesion: 0.22
Nodes (9): Fab(), SpeedDial(), SpeedDialAction(), SpeedDialContext, SpeedDialDirection, Down, Left, Right (+1 more)

### Community 65 - "pie.rs"
Cohesion: 0.23
Nodes (5): GaugeChart(), RadialBar(), DonutChart(), PieChart(), PieSegment

### Community 66 - "TreeNode"
Cohesion: 0.23
Nodes (3): TreeNode, TreeNodeRow(), TreeView()

### Community 68 - "src/drawer.rs"
Cohesion: 0.24
Nodes (8): Drawer(), DrawerPosition, Left, Right, DrawerSize, Lg, Md, Sm

### Community 69 - "modal.rs"
Cohesion: 0.18
Nodes (7): Modal(), ModalSize, Lg, Md, Sm, Xl, Xxl

### Community 70 - "ScrollType"
Cohesion: 0.25
Nodes (7): ScrollArea(), ScrollAreaProps, ScrollType, Always, Auto, Hover, Never

### Community 73 - "AffixPosition"
Cohesion: 0.29
Nodes (7): Affix(), AffixPosition, BottomLeft, BottomRight, TopLeft, TopRight, BackToTop()

### Community 74 - "src/media_card.rs"
Cohesion: 0.27
Nodes (3): AspectRatio(), CardActionArea(), MediaCard()

### Community 75 - "Burger"
Cohesion: 0.24
Nodes (5): Burger(), BurgerSize, Lg, Md, Sm

### Community 76 - "CheckboxSize"
Cohesion: 0.27
Nodes (3): Checkbox(), CheckboxSize, CheckboxSizeExt

### Community 77 - "src/context_menu.rs"
Cohesion: 0.33
Nodes (7): ContextMenu(), ContextMenuCheckboxItem(), ContextMenuContent(), ContextMenuContext, ContextMenuItem(), ContextMenuSeparator(), ContextMenuTrigger()

### Community 79 - "src/number_input.rs"
Cohesion: 0.31
Nodes (6): format_and_clamp_number(), format_number_string(), NumberInput(), test_format_and_clamp_over_max(), test_format_and_clamp_under_min(), test_format_and_clamp_within_bounds()

### Community 80 - "RadiusScale"
Cohesion: 0.22
Nodes (4): RadiusScale, Medium, Sharp, Soft

### Community 81 - "check_classes.py"
Cohesion: 0.33
Nodes (5): compile_scss(), extract_selectors(), main(), sass_command(), scan_source_classes()

### Community 82 - "📦 Libreria UI (`crate/`)"
Cohesion: 0.20
Nodes (9): Avviare l'Espositore, 🖼️ Espositore (`showcase/`), Feature disponibili, Includere la libreria nei tuoi siti web, 🎨 Integrazione Asset SCSS (`STYLES`), 📦 Libreria UI (`crate/`), Shadow Design, 🎭 Sistema Temi Mantine-Style & ThemeBuilder (+1 more)

### Community 83 - "BarChart"
Cohesion: 0.22
Nodes (8): BarChart(), BarChartType, Default, Percent, Stacked, ChartOrientation, Horizontal, Vertical

### Community 84 - "GlowMode"
Cohesion: 0.28
Nodes (6): GlowMode, Expressive, Off, Subtle, GlowTokens, test_glow_modes_different()

### Community 85 - "heatmap.rs"
Cohesion: 0.36
Nodes (3): CalendarHeatmap(), Heatmap(), HeatmapCell

### Community 86 - "FieldsetVariant"
Cohesion: 0.32
Nodes (5): Fieldset(), FieldsetVariant, Default, Filled, Ungrouped

### Community 87 - "nav.rs"
Cohesion: 0.32
Nodes (6): CATEGORIES, ComponentCategory, ComponentItem, ComponentNavEntry, COMPONENTS, SearchPalette()

### Community 88 - "SkeletonShape"
Cohesion: 0.32
Nodes (5): Skeleton(), SkeletonShape, Circle, Rectangle, Text

### Community 89 - "ElevationStyle"
Cohesion: 0.29
Nodes (5): ElevationStyle, Flat, HardBlock, NeonGlow, SoftElevated

### Community 91 - "layout.rs"
Cohesion: 0.29
Nodes (5): VERSION, ColorSwatchButton(), Layout(), SidebarLink(), SidebarView()

### Community 93 - "Shadow Design (`shadow-design`)"
Cohesion: 0.29
Nodes (6): 💻 Esempio d'Uso, 🧩 Feature Flags Disponibili, 📦 Installazione in Cargo.toml, 🎨 Integrazione Diretta Asset SCSS in Dioxus 0.7, Shadow Design (`shadow-design`), 🎨 Sistema del Tema Mantine (`theme`)

### Community 94 - "ColorFormat"
Cohesion: 0.33
Nodes (6): ColorFormat, Hex, Hsl, Rgb, ColorInputProps, ColorPickerProps

### Community 95 - "Orientation"
Cohesion: 0.38
Nodes (4): Divider(), Orientation, Horizontal, Vertical

### Community 96 - "src/rating.rs"
Cohesion: 0.20
Nodes (4): Icon(), LoadingOverlay(), clamp_rating_value(), Rating()

### Community 101 - "src/tabs.rs"
Cohesion: 0.43
Nodes (5): Tab(), TabList(), TabListCtx, TabPanel(), Tabs()

### Community 105 - "RatioPreset"
Cohesion: 0.33
Nodes (5): RatioPreset, Cinematic21_9, Square1_1, Standard4_3, Widescreen16_9

### Community 108 - "src/highlight.rs"
Cohesion: 0.60
Nodes (3): Chunk, Highlight(), split_highlight()

### Community 113 - "UseWindowScroll"
Cohesion: 0.47
Nodes (3): use_window_scroll(), UseWindowScroll, test_window_scroll_initial_value()

### Community 115 - "Shadow Design — Showcase App"
Cohesion: 0.50
Nodes (3): 🎨 Architettura Stili, Shadow Design — Showcase App, 🚀 Sviluppo Locale (Integrazione Diretta Dioxus 0.7)

### Community 162 - "reactive_hooks_tests.rs"
Cohesion: 0.40
Nodes (4): use_clipboard(), test_clipboard_initial_state(), test_form_validation_and_errors(), TestFormModel

### Community 164 - "CodeBlock"
Cohesion: 0.50
Nodes (3): CodeBlock, all_cargo_features(), Home()

### Community 172 - "src/coachmark.rs"
Cohesion: 0.29
Nodes (7): Coachmark(), CoachmarkPosition, Bottom, Left, Right, Top, TeachingBubble()

### Community 177 - "nav_link/mod.rs"
Cohesion: 0.31
Nodes (4): SidebarMenu(), SidebarMenuProps, NavLink(), NavLinkProps

## Knowledge Gaps
- **392 isolated node(s):** `shadow-design`, `Sm`, `Md`, `Lg`, `Default` (+387 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1034 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **88 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `IconKind` connect `IconKind` to `avatar.rs`, `notification.rs`, `ButtonVariant`, `src/rich_text.rs`, `complex_tooling_tests.rs`, `SvgIcon`, `menubar/mod.rs`, `stepper/mod.rs`, `src/overflow_set.rs`, `accordion/mod.rs`, `ControlSize`, `use_theme`, `ComponentSpacing`, `PersonaProps`, `src/select.rs`, `AlertVariant`, `breadcrumbs/mod.rs`, `src/chip.rs`, `src/coachmark.rs`, `CloseButton`, `Signal`, `DeliveryStatus`, `nav_link/mod.rs`, `NavigationLinkItem`, `src/speed_dial.rs`, `TreeNode`, `AffixPosition`, `CheckboxSize`, `src/context_menu.rs`, `src/number_input.rs`, `nav.rs`, `src/transfer_list.rs`, `layout.rs`, `src/rating.rs`, `src/tabs.rs`?**
  _High betweenness centrality (0.071) - this node is a cross-community bridge._
- **What connects `shadow-design`, `Sm`, `Md` to the rest of the system?**
  _392 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `src/main.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05710162853019996 - nodes in this community are weakly interconnected._
- **Why does `Signal` connect `Signal` to `IconKind`, `notification.rs`, `ButtonVariant`, `src/rich_text.rs`, `table.rs`, `SvgIcon`, `menubar/mod.rs`, `app_shell/mod.rs`, `src/overflow_set.rs`, `accordion/mod.rs`, `ControlSize`, `src/slider.rs`, `use_theme`, `PopoverPosition`, `src/select.rs`, `breadcrumbs/mod.rs`, `src/chip.rs`, `super`, `PrimaryColor`, `nav_link/mod.rs`, `UseForm<T>`, `hooks/mod.rs`, `area.rs`, `src/pin_input.rs`, `HoverCardPosition`, `SegmentedControl`, `src/speed_dial.rs`, `TreeNode`, `src/drawer.rs`, `modal.rs`, `debounce.rs`, `Burger`, `CheckboxSize`, `src/context_menu.rs`, `Locale`, `src/number_input.rs`, `nav.rs`, `src/transfer_list.rs`, `ColorFormat`, `src/rating.rs`, `src/tabs.rs`, `UseWindowScroll`, `Switch`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **Should `lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.03333333333333333 - nodes in this community are weakly interconnected._
- **Why does `Theme` connect `Theme` to `lib.rs`, `notification.rs`, `ButtonVariant`, `AlertVariant`, `CardShadow`, `PrimaryColor`, `RadiusScale`, `GlowMode`, `BadgeVariant`, `PrimaryColor`, `ElevationStyle`, `use_theme`, `ThemeBuilder`?**
  _High betweenness centrality (0.027) - this node is a cross-community bridge._
- **Should `IconKind` be split into smaller, more focused modules?**
  _Cohesion score 0.03571428571428571 - nodes in this community are weakly interconnected._