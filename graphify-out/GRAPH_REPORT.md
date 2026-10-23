# Graph Report - shadow-design  (2026-10-23)

## Corpus Check
- 255 files · ~168,584 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 106 file(s) not represented in the graph (top: .scss 94, (none) 7, .toml 2)

## Summary
- 2217 nodes · 3877 edges · 186 communities (94 shown, 92 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 65 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `02f94e69`
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
- Icon
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
- Date
- IndicatorColor
- ComponentSpacing
- Theme
- timeline.rs
- HeadingOrder
- AGENTS.md — Shadow Design Workspace Guide
- primitives.rs
- use_theme
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
- theme.rs
- TokenKind
- reactive_hooks_tests.rs
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
- ComponentOverride
- UseClipboard
- bottom_navigation/mod.rs
- HoverCardPosition
- SegmentedControl
- src/speed_dial.rs
- pie.rs
- TreeNode
- super
- src/drawer.rs
- modal.rs
- ScrollType
- ThemeBuilder
- AffixPosition
- src/media_card.rs
- Burger
- CheckboxSize
- src/context_menu.rs
- use_locale
- src/number_input.rs
- RadiusScale
- check_classes.py
- 📦 Libreria UI (`crate/`)
- BarChart
- CommandAction
- heatmap.rs
- FieldsetVariant
- glow.rs
- SkeletonShape
- ElevationStyle
- src/transfer_list.rs
- layout.rs
- debounce.rs
- Shadow Design (`shadow-design`)
- ColorFormat
- Orientation
- src/rating.rs
- src/textarea.rs
- src/dropzone.rs
- UseWindowScroll
- src/tabs.rs
- RatioPreset
- RadarChart
- ContactCard
- src/highlight.rs
- click_outside.rs
- src/center.rs
- src/copy_button.rs
- Switch
- Shadow Design — Showcase App
- tables.rs
- CommandPalette
- src/loading_overlay.rs
- accordion.rs
- pages/activity_feed.rs
- pages/affix.rs
- pages/alert_dialog.rs
- app_shell.rs
- pages/aspect_ratio.rs
- attachment_cards.rs
- bottom_navigation.rs
- breadcrumbs.rs
- pages/burger.rs
- pages/carousel.rs
- pages/center.rs
- charts.rs
- pages/chat.rs
- pages/code_highlight.rs
- pages/color_picker.rs
- pages/context_menu.rs
- pages/copy_button.rs
- pages/date_picker.rs
- dividers.rs
- pages/document_card.rs
- pages/drawer.rs
- pages/dropdown.rs
- pages/dropzone.rs
- pages/facepile.rs
- pages/grid.rs
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
- nav_link.rs
- pages/navbar.rs
- pages/navigation_menu.rs
- pages/number_input.rs
- pages/overflow_set.rs
- pagination.rs
- pages/popover.rs
- pages/progress.rs
- pages/radio.rs
- pages/rating.rs
- pages/resizable.rs
- pages/rich_text.rs
- scroll_area.rs
- segmented_controls.rs
- pages/speed_dial.rs
- spoiler.rs
- stepper.rs
- pages/switch.rs
- pages/tabs.rs
- pages/textarea.rs
- pages/tooltip.rs
- pages/transfer_list.rs
- trees.rs
- pages/typography.rs
- pages/virtual_list.rs
- STYLES
- GEMINI.md
- shadow-design
- renovate.json

## God Nodes (most connected - your core abstractions)
1. `IconKind` - 136 edges
2. `Route` - 94 edges
3. `PropRow` - 92 edges
4. `ShowcaseTabs()` - 92 edges
5. `FeatureBanner()` - 89 edges
6. `Signal` - 76 edges
7. `Icon()` - 53 edges
8. `Theme` - 47 edges
9. `SvgIcon` - 32 edges
10. `use_theme()` - 28 edges

## Surprising Connections (you probably didn't know these)
- `ReactiveHooksPage()` --calls--> `use_clipboard()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/clipboard.rs
- `ReactiveHooksPage()` --calls--> `use_debounced_signal()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/debounce.rs
- `ReactiveHooksPage()` --calls--> `use_window_scroll()`  [INFERRED]
  showcase/src/pages/reactive_hooks.rs → crate/src/hooks/scroll.rs
- `FeatureBanner()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/components/banner.rs → crate/src/theme.rs
- `Layout()` --calls--> `use_theme()`  [INFERRED]
  showcase/src/components/layout.rs → crate/src/theme.rs

## Import Cycles
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

## Communities (186 total, 92 thin omitted)

### Community 0 - "src/main.rs"
Cohesion: 0.05
Nodes (96): ComponentNavEntry, COMPONENTS, App(), FAVICON, HEADER_SVG, Route, AccordionPage, ActivityFeedPage (+88 more)

### Community 2 - "IconKind"
Cohesion: 0.04
Nodes (55): AccordionControl(), IconKind, AlertCircle, AlertTriangle, .ALL, Bluetooth, Box, Calendar (+47 more)

### Community 3 - "avatar.rs"
Cohesion: 0.06
Nodes (20): ActivityComment, ActivityEvent, ActivityFeed(), ActivityItem(), Avatar(), AvatarGroup(), AvatarSize, Lg (+12 more)

### Community 4 - "notification.rs"
Cohesion: 0.07
Nodes (23): NotificationContainer(), NotificationHandle, NotificationItem, NotificationProvider(), NotificationToast(), NotificationVariant, Danger, Info (+15 more)

### Community 5 - "ButtonVariant"
Cohesion: 0.06
Nodes (24): AlertDialog(), ConfirmDialog(), Button(), ButtonSize, Lg, Md, Sm, Xl (+16 more)

### Community 6 - "AnimationVariant"
Cohesion: 0.05
Nodes (37): Animations, .DRAWER_LEFT, .DRAWER_RIGHT, .DROPDOWN_ENTER, .FADE_IN, .FADE_OUT, .INDETERMINATE, .MODAL_ENTER (+29 more)

### Community 7 - "ListMarker"
Cohesion: 0.07
Nodes (32): List(), ListContext, ListItem(), ListMarker, Circle, Custom, Decimal, Disc (+24 more)

### Community 8 - "src/rich_text.rs"
Cohesion: 0.08
Nodes (29): format_inline(), html_escape(), html_to_markdown(), markdown_to_html(), RichTextCommand, Blockquote, Bold, BulletList (+21 more)

### Community 9 - "JustifyContent"
Cohesion: 0.07
Nodes (22): AlignItems, Baseline, Center, End, Start, Stretch, Group(), JustifyContent (+14 more)

### Community 10 - "table.rs"
Cohesion: 0.09
Nodes (17): DataTable(), DataTableColumn, DataTableColumn<T>, SortDirection, Ascending, Descending, None, Table() (+9 more)

### Community 11 - "complex_tooling_tests.rs"
Cohesion: 0.06
Nodes (14): CodeBlock(), CodeHighlight(), SyntaxToken, SyntaxTokenKind, Comment, Function, Keyword, Number (+6 more)

### Community 12 - "GridSpacing"
Cohesion: 0.08
Nodes (22): Grid(), GridAlign, Baseline, Center, End, Start, Stretch, GridCol() (+14 more)

### Community 13 - "Icon"
Cohesion: 0.13
Nodes (10): AttachmentCard(), BottomNavItem(), BottomNavItemProps, Icon(), render_icon_paths(), RenderSvgIcon(), SvgIcon, Custom (+2 more)

### Community 14 - "menubar/mod.rs"
Cohesion: 0.14
Nodes (16): MenubarItem(), MenubarItemProps, MenubarContent(), MenubarContentProps, MenubarMenu(), MenubarMenuProps, MenubarTrigger(), MenubarTriggerProps (+8 more)

### Community 15 - "stepper/mod.rs"
Cohesion: 0.11
Nodes (16): StepperCompleted(), StepperCompletedProps, Stepper(), StepperCtx, StepperOrientation, .ALL, Horizontal, Vertical (+8 more)

### Community 16 - "app_shell/mod.rs"
Cohesion: 0.14
Nodes (13): AppShellAside(), AppShellAsideProps, AppShellFooter(), AppShellFooterProps, AppShellHeader(), AppShellHeaderProps, AppShellMain(), AppShellMainProps (+5 more)

### Community 17 - "prelude"
Cohesion: 0.08
Nodes (8): Keytip(), Navbar(), copy_text(), AnimationsPage(), CheckboxPage(), CARGO_FEATURES, LoadingOverlayPage(), PinInputPage()

### Community 18 - "src/color_picker.rs"
Cohesion: 0.18
Nodes (20): ColorInput(), ColorPicker(), default_color_swatches(), format_color(), hsl_to_rgba(), Hsla, hsv_to_rgb(), parse_color() (+12 more)

### Community 19 - "src/overflow_set.rs"
Cohesion: 0.11
Nodes (12): Dropdown(), DropdownCtx, DropdownDivider(), DropdownItem(), DropdownPlacement, BottomLeft, BottomRight, TopLeft (+4 more)

### Community 20 - "accordion/mod.rs"
Cohesion: 0.12
Nodes (12): AccordionItem(), Accordion(), AccordionContext, AccordionItemContext, AccordionVariant, Contained, Default, Separated (+4 more)

### Community 21 - "BadgeVariant"
Cohesion: 0.12
Nodes (12): Badge(), BadgeVariant, Danger, Default, Outline, Primary, Success, Warning (+4 more)

### Community 22 - "FeatureBanner"
Cohesion: 0.08
Nodes (8): FeatureBanner(), AvatarsPage(), ButtonsPage(), CardsPage(), FieldsetPage(), UserRegistrationModel, MenubarPage(), StackPage()

### Community 23 - "ShowcaseTabs"
Cohesion: 0.08
Nodes (8): ShowcaseTabs(), AlertsPage(), ChipPage(), CloseButtonPage(), CommandPalettePage(), SkeletonsPage(), SliderPage(), TimelinesPage()

### Community 24 - "ControlSize"
Cohesion: 0.15
Nodes (15): ControlSize, Lg, Md, Sm, calculate_pagination_range(), PaginationControl(), PaginationControlProps, PaginationItem (+7 more)

### Community 25 - "src/slider.rs"
Cohesion: 0.14
Nodes (12): calc_percentage(), RangeSlider(), Slider(), SliderMark, snap_to_step(), test_slider_marks(), Tooltip(), TooltipPosition (+4 more)

### Community 26 - "Date"
Cohesion: 0.17
Nodes (8): Calendar(), CalendarViewMode, Day, Month, Year, Date, DatePicker(), DateRangePicker()

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
Cohesion: 0.10
Nodes (18): 1. Panoramica del Progetto, 2. Struttura del Workspace Cargo, 3. Crate Libreria (`crate` / `shadow-design`), 4. Crate Showcase (`showcase`), 5. Regole e Convenzioni per lo Sviluppo, 6. Comandi Utili per il Workflow, AGENTS.md — Shadow Design Workspace Guide, Architettura 100% SCSS & Supporto Nativo Dioxus `asset!()` nella Crate (+10 more)

### Community 33 - "primitives.rs"
Cohesion: 0.17
Nodes (16): CurveType, Linear, Monotone, Step, DEFAULT_CHART_COLORS, generate_stacked_area_band_path(), generate_svg_area_path(), generate_svg_path() (+8 more)

### Community 34 - "use_theme"
Cohesion: 0.15
Nodes (11): Carousel(), CarouselSlide(), ResizableDirection, Horizontal, Vertical, ResizableHandle(), ResizablePanel(), ResizablePanelGroup() (+3 more)

### Community 35 - "PopoverPosition"
Cohesion: 0.13
Nodes (17): Popover(), PopoverContext, PopoverDropdown(), PopoverPosition, Bottom, BottomEnd, BottomStart, Left (+9 more)

### Community 36 - "PropRow"
Cohesion: 0.13
Nodes (10): CodeBlock, PropRow, PropsTable(), ShowcaseTab, Api, Code, Playground, BadgesPage() (+2 more)

### Community 37 - "src/select.rs"
Cohesion: 0.24
Nodes (10): Combobox(), display_label(), filter_items(), MultiSelect(), Select(), SelectItem, SelectOption(), test_display_label() (+2 more)

### Community 38 - "Changelog — Shadow Design"
Cohesion: 0.11
Nodes (18): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti, 🐛 Correzioni & Ottimizzazioni (+10 more)

### Community 39 - "AlertVariant"
Cohesion: 0.15
Nodes (6): Alert(), AlertVariant, Danger, Info, Success, Warning

### Community 40 - "breadcrumbs/mod.rs"
Cohesion: 0.16
Nodes (10): BreadcrumbItem(), BreadcrumbItemProps, BreadcrumbData, Breadcrumbs(), BreadcrumbsProps, calculate_collapse_bounds(), test_breadcrumb_data_builder(), test_collapse_bounds_custom_overrides() (+2 more)

### Community 41 - "CardShadow"
Cohesion: 0.12
Nodes (12): Card(), CardShadow, Lg, Md, None, Sm, Xl, CardVariant (+4 more)

### Community 42 - "ChartSeries"
Cohesion: 0.14
Nodes (6): AreaChart(), LineChart(), ChartDataPoint, ChartSeries, compute_min_max(), test_point_builder()

### Community 43 - "src/chip.rs"
Cohesion: 0.18
Nodes (11): Chip(), ChipGroup(), ChipGroupContext, ChipGroupProps, ChipProps, ChipVariant, Filled, Light (+3 more)

### Community 44 - "Changelog — Shadow Design"
Cohesion: 0.11
Nodes (17): [0.1.0] — 2026-10-09, [0.2.0] — 2026-10-11, [0.3.0] — 2026-10-14, [0.4.0] — 2026-10-20, 📦 Architettura del Crate & Metadati, Changelog — Shadow Design, 🐛 Correzioni & Miglioramenti, 🐛 Correzioni & Ottimizzazioni (+9 more)

### Community 45 - "theme.rs"
Cohesion: 0.15
Nodes (8): GlowMode, Expressive, Off, Subtle, GlowTokens, test_cartoon_apply_primary_color_keeps_components(), test_glow_modes_different(), test_primary_color_custom_with_prefix_sanitized()

### Community 46 - "TokenKind"
Cohesion: 0.14
Nodes (14): CodeBlock(), CopyState, Copied, Failed, Idle, tokenize_rsx(), TokenKind, Comment (+6 more)

### Community 47 - "reactive_hooks_tests.rs"
Cohesion: 0.17
Nodes (7): FormOptions, use_form(), UseForm, test_debounced_signal_initial_value(), test_form_validation_and_errors(), TestFormModel, FormValidationPage()

### Community 48 - "CloseButton"
Cohesion: 0.16
Nodes (9): CloseButton(), CloseButtonSize, Lg, Md, Sm, CloseButtonVariant, Filled, Subtle (+1 more)

### Community 49 - "Signal"
Cohesion: 0.19
Nodes (9): Signal, Radio(), RadioCard(), RadioContext, RadioGroup(), RadioOrientation, Horizontal, Vertical (+1 more)

### Community 50 - "DeliveryStatus"
Cohesion: 0.16
Nodes (9): BubbleDirection, Incoming, Outgoing, DeliveryStatus, Delivered, Failed, Sending, Sent (+1 more)

### Community 52 - "NavigationLinkItem"
Cohesion: 0.21
Nodes (5): NavigationLinkItem, NavigationMenu(), NavigationMenuItem, NavigationMenuSection, NavigationPreviewCard

### Community 53 - "ProgressVariant"
Cohesion: 0.16
Nodes (10): ProgressBar(), ProgressSize, Lg, Md, Sm, ProgressVariant, Danger, Primary (+2 more)

### Community 54 - "PrimaryColor"
Cohesion: 0.15
Nodes (8): ColorTokens, PrimaryColor, Crimson, Cyan, Emerald, Obsidian, Violet, ThemeProvider()

### Community 55 - "hooks/mod.rs"
Cohesion: 0.18
Nodes (6): use_hotkey(), use_idle(), use_intersection_observer(), use_local_storage(), use_media_query(), ReactiveHooksPage()

### Community 56 - "area.rs"
Cohesion: 0.33
Nodes (4): ChartContainer(), ChartGrid(), ChartLegend(), ChartTooltip()

### Community 57 - "src/pin_input.rs"
Cohesion: 0.18
Nodes (7): use_unique_id(), PinInput(), PinInputProps, PinInputType, Alphanumeric, Numeric, sanitize_pin_value()

### Community 58 - "get_series_color"
Cohesion: 0.20
Nodes (7): get_series_color(), BubbleChart(), ScaleType, Linear, Log, ScatterChart(), ScatterPoint

### Community 59 - "ComponentOverride"
Cohesion: 0.18
Nodes (4): ComponentOverride, Custom, test_builder_preserves_overrides_when_changing_color(), test_primary_color_custom_classes()

### Community 60 - "UseClipboard"
Cohesion: 0.21
Nodes (5): use_clipboard(), UseClipboard, MarqueeRect, MarqueeSelection(), test_clipboard_initial_state()

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
Cohesion: 0.20
Nodes (7): Modal(), ModalSize, Lg, Md, Sm, Xl, Xxl

### Community 70 - "ScrollType"
Cohesion: 0.25
Nodes (7): ScrollArea(), ScrollAreaProps, ScrollType, Always, Auto, Hover, Never

### Community 71 - "ThemeBuilder"
Cohesion: 0.25
Nodes (3): hex_to_rgb(), is_valid_hex_color(), ThemeBuilder

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
Cohesion: 0.24
Nodes (5): Checkbox(), CheckboxSize, Lg, Md, Sm

### Community 77 - "src/context_menu.rs"
Cohesion: 0.33
Nodes (7): ContextMenu(), ContextMenuCheckboxItem(), ContextMenuContent(), ContextMenuContext, ContextMenuItem(), ContextMenuSeparator(), ContextMenuTrigger()

### Community 78 - "use_locale"
Cohesion: 0.29
Nodes (3): Locale, LocaleProvider(), use_locale()

### Community 79 - "src/number_input.rs"
Cohesion: 0.31
Nodes (6): format_and_clamp_number(), format_number_string(), NumberInput(), test_format_and_clamp_over_max(), test_format_and_clamp_under_min(), test_format_and_clamp_within_bounds()

### Community 80 - "RadiusScale"
Cohesion: 0.22
Nodes (4): RadiusScale, Medium, Sharp, Soft

### Community 81 - "check_classes.py"
Cohesion: 0.38
Nodes (4): compile_scss(), extract_selectors(), main(), scan_source_classes()

### Community 82 - "📦 Libreria UI (`crate/`)"
Cohesion: 0.20
Nodes (9): Avviare l'Espositore, 🖼️ Espositore (`showcase/`), Feature disponibili, Includere la libreria nei tuoi siti web, 🎨 Integrazione Asset SCSS (`STYLES`), 📦 Libreria UI (`crate/`), Shadow Design, 🎭 Sistema Temi Mantine-Style & ThemeBuilder (+1 more)

### Community 83 - "BarChart"
Cohesion: 0.22
Nodes (8): BarChart(), BarChartType, Default, Percent, Stacked, ChartOrientation, Horizontal, Vertical

### Community 85 - "heatmap.rs"
Cohesion: 0.36
Nodes (3): CalendarHeatmap(), Heatmap(), HeatmapCell

### Community 86 - "FieldsetVariant"
Cohesion: 0.32
Nodes (5): Fieldset(), FieldsetVariant, Default, Filled, Ungrouped

### Community 87 - "glow.rs"
Cohesion: 0.36
Nodes (4): ComponentGlowSpec, get_spec(), resolve_glow(), SPECS

### Community 88 - "SkeletonShape"
Cohesion: 0.32
Nodes (5): Skeleton(), SkeletonShape, Circle, Rectangle, Text

### Community 89 - "ElevationStyle"
Cohesion: 0.29
Nodes (5): ElevationStyle, Flat, HardBlock, NeonGlow, SoftElevated

### Community 91 - "layout.rs"
Cohesion: 0.39
Nodes (4): ColorSwatchButton(), Layout(), SidebarLink(), SidebarView()

### Community 92 - "debounce.rs"
Cohesion: 0.43
Nodes (3): sleep_ms(), use_debounced_signal(), use_throttle()

### Community 93 - "Shadow Design (`shadow-design`)"
Cohesion: 0.29
Nodes (6): 💻 Esempio d'Uso, 🧩 Feature Flags Disponibili, 📦 Installazione in Cargo.toml, 🎨 Integrazione Diretta Asset SCSS in Dioxus 0.7, Shadow Design (`shadow-design`), 🎨 Sistema del Tema Mantine (`theme`)

### Community 94 - "ColorFormat"
Cohesion: 0.33
Nodes (6): ColorFormat, Hex, Hsl, Rgb, ColorInputProps, ColorPickerProps

### Community 95 - "Orientation"
Cohesion: 0.38
Nodes (4): Divider(), Orientation, Horizontal, Vertical

### Community 100 - "UseWindowScroll"
Cohesion: 0.47
Nodes (3): use_window_scroll(), UseWindowScroll, test_window_scroll_initial_value()

### Community 101 - "src/tabs.rs"
Cohesion: 0.47
Nodes (4): Tab(), TabList(), TabListCtx, TabPanel()

### Community 105 - "RatioPreset"
Cohesion: 0.33
Nodes (5): RatioPreset, Cinematic21_9, Square1_1, Standard4_3, Widescreen16_9

### Community 108 - "src/highlight.rs"
Cohesion: 0.60
Nodes (3): Chunk, Highlight(), split_highlight()

### Community 115 - "Shadow Design — Showcase App"
Cohesion: 0.50
Nodes (3): 🎨 Architettura Stili, Shadow Design — Showcase App, 🚀 Sviluppo Locale (Integrazione Diretta Dioxus 0.7)

## Knowledge Gaps
- **407 isolated node(s):** `shadow-design`, `Sm`, `Md`, `Lg`, `Default` (+402 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1015 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **92 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `IconKind` connect `IconKind` to `src/main.rs`, `avatar.rs`, `notification.rs`, `ButtonVariant`, `src/rich_text.rs`, `complex_tooling_tests.rs`, `Icon`, `src/color_picker.rs`, `src/overflow_set.rs`, `BadgeVariant`, `ControlSize`, `Date`, `ComponentSpacing`, `use_theme`, `src/select.rs`, `AlertVariant`, `breadcrumbs/mod.rs`, `src/chip.rs`, `CloseButton`, `Signal`, `DeliveryStatus`, `NavigationLinkItem`, `src/speed_dial.rs`, `TreeNode`, `src/drawer.rs`, `modal.rs`, `AffixPosition`, `CheckboxSize`, `src/context_menu.rs`, `src/number_input.rs`, `CommandAction`, `src/transfer_list.rs`, `layout.rs`, `src/rating.rs`, `src/textarea.rs`, `src/dropzone.rs`, `src/tabs.rs`, `src/copy_button.rs`, `src/loading_overlay.rs`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **What connects `shadow-design`, `Sm`, `Md` to the rest of the system?**
  _407 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `src/main.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.054746651135701804 - nodes in this community are weakly interconnected._
- **Why does `Signal` connect `Signal` to `IconKind`, `notification.rs`, `ButtonVariant`, `src/rich_text.rs`, `table.rs`, `Icon`, `menubar/mod.rs`, `app_shell/mod.rs`, `src/overflow_set.rs`, `accordion/mod.rs`, `BadgeVariant`, `ControlSize`, `src/slider.rs`, `Date`, `use_theme`, `PopoverPosition`, `src/select.rs`, `breadcrumbs/mod.rs`, `src/chip.rs`, `reactive_hooks_tests.rs`, `PrimaryColor`, `hooks/mod.rs`, `area.rs`, `src/pin_input.rs`, `UseClipboard`, `HoverCardPosition`, `SegmentedControl`, `src/speed_dial.rs`, `TreeNode`, `src/drawer.rs`, `modal.rs`, `Burger`, `CheckboxSize`, `src/context_menu.rs`, `use_locale`, `src/number_input.rs`, `src/transfer_list.rs`, `debounce.rs`, `ColorFormat`, `src/rating.rs`, `src/textarea.rs`, `UseWindowScroll`, `src/tabs.rs`, `Switch`, `CommandPalette`?**
  _High betweenness centrality (0.055) - this node is a cross-community bridge._
- **Should `lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.03333333333333333 - nodes in this community are weakly interconnected._
- **Should `IconKind` be split into smaller, more focused modules?**
  _Cohesion score 0.03508771929824561 - nodes in this community are weakly interconnected._
- **Should `avatar.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.060408163265306125 - nodes in this community are weakly interconnected._