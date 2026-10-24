use dioxus::prelude::*;
use shadow_design::prelude::*;
pub use shadow_design::VERSION;

mod components;
mod pages;

use components::Layout;
use pages::{
    AccordionPage, AlertsPage, AnimationsPage, AppShellPage, AspectRatioPage, AttachmentCardsPage, AvatarsPage, BadgesPage, BottomNavigationPage, BreadcrumbsPage, ButtonsPage, CardsPage, ChatPage,
    CheckboxPage, DividersPage, DrawerPage, DropdownPage, GridPage, GroupPage, Home, IconsPage, InputsPage, ListsPage, MenubarPage, ModalsPage,
    CenterPage,
    ChipPage, ColorPickerPage, NavbarDemoPage, NavLinkPage, NotificationsPage, NumberInputPage, PaginationPage, PinInputPage, ProgressPage, RadioPage, RatingPage, ScrollAreaPage, SegmentedControlsPage, SelectPage,
    SkeletonsPage, SliderPage, SpoilerPage, StackPage, StepperPage, SwitchPage, TablesPage, TabsPage, TextareaPage, ThemeBuilderPage, TimelinesPage, TooltipPage, TreesPage,
    TypographyPage,
    PopoverPage, HoverCardPage, ContextMenuPage, AlertDialogPage, CoachmarkPage, SpeedDialPage, AffixPage,
    BurgerPage, CloseButtonPage, CopyButtonPage, IndicatorPage, HighlightPage, LoadingOverlayPage, FieldsetPage, MarqueeSelectionPage, KeytipPage,
    FormValidationPage, ReactiveHooksPage,
    CommandPalettePage, DatePickerPage, DropzonePage, ResizablePage, CarouselPage, VirtualListPage, CodeHighlightPage,
    ChartsPage, RichTextPage, DocumentCardPage, MasonryPage, MediaCardPage,
    NavigationMenuPage, OverflowSetPage, TransferListPage, FacepilePage, ActivityFeedPage,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/theme-builder")]
    ThemeBuilderPage {},
    #[route("/typography")]
    TypographyPage {},
    #[route("/buttons")]
    ButtonsPage {},
    #[route("/cards")]
    CardsPage {},
    #[route("/inputs")]
    InputsPage {},
    #[route("/number-input")]
    NumberInputPage {},
    #[route("/radio")]
    RadioPage {},
    #[route("/select")]
    SelectPage {},
    #[route("/textarea")]
    TextareaPage {},
    #[route("/checkbox")]
    CheckboxPage {},
    #[route("/slider")]
    SliderPage {},
    #[route("/rating")]
    RatingPage {},
    #[route("/badges")]
    BadgesPage {},
    #[route("/alerts")]
    AlertsPage {},
    #[route("/modals")]
    ModalsPage {},
    #[route("/navbar")]
    NavbarDemoPage {},
    #[route("/app-shell")]
    AppShellPage {},
    #[route("/aspect-ratio")]
    AspectRatioPage {},
    #[route("/notifications")]
    NotificationsPage {},
    #[route("/tabs")]
    TabsPage {},
    #[route("/switch")]
    SwitchPage {},
    #[route("/tooltip")]
    TooltipPage {},
    #[route("/drawer")]
    DrawerPage {},
    #[route("/dropdown")]
    DropdownPage {},
    #[route("/avatars")]
    AvatarsPage {},
    #[route("/chat")]
    ChatPage {},
    #[route("/progress")]
    ProgressPage {},
    #[route("/attachment-cards")]
    AttachmentCardsPage {},
    #[route("/segmented-controls")]
    SegmentedControlsPage {},
    #[route("/dividers")]
    DividersPage {},
    #[route("/grid")]
    GridPage {},
    #[route("/group")]
    GroupPage {},
    #[route("/stack")]
    StackPage {},
    #[route("/center")]
    CenterPage {},
    #[route("/trees")]
    TreesPage {},
    #[route("/skeletons")]
    SkeletonsPage {},
    #[route("/lists")]
    ListsPage {},
    #[route("/timelines")]
    TimelinesPage {},
    #[route("/tables")]
    TablesPage {},
    #[route("/scroll-area")]
    ScrollAreaPage {},
    #[route("/breadcrumbs")]
    BreadcrumbsPage {},
    #[route("/pagination")]
    PaginationPage {},
    #[route("/nav-link")]
    NavLinkPage {},
    #[route("/bottom-navigation")]
    BottomNavigationPage {},
    #[route("/stepper")]
    StepperPage {},
    #[route("/menubar")]
    MenubarPage {},
    #[route("/icons")]
    IconsPage {},
    #[route("/animations")]
    AnimationsPage {},
    #[route("/pin-input")]
    PinInputPage {},
    #[route("/color-picker")]
    ColorPickerPage {},
    #[route("/chip")]
    ChipPage {},
    #[route("/accordion")]
    AccordionPage {},
    #[route("/spoiler")]
    SpoilerPage {},
    #[route("/popover")]
    PopoverPage {},
    #[route("/hover-card")]
    HoverCardPage {},
    #[route("/context-menu")]
    ContextMenuPage {},
    #[route("/alert-dialog")]
    AlertDialogPage {},
    #[route("/coachmark")]
    CoachmarkPage {},
    #[route("/speed-dial")]
    SpeedDialPage {},
    #[route("/affix")]
    AffixPage {},
    #[route("/burger")]
    BurgerPage {},
    #[route("/close-button")]
    CloseButtonPage {},
    #[route("/copy-button")]
    CopyButtonPage {},
    #[route("/indicator")]
    IndicatorPage {},
    #[route("/highlight")]
    HighlightPage {},
    #[route("/loading-overlay")]
    LoadingOverlayPage {},
    #[route("/fieldset")]
    FieldsetPage {},
    #[route("/marquee-selection")]
    MarqueeSelectionPage {},
    #[route("/keytip")]
    KeytipPage {},
    #[route("/form")]
    FormValidationPage {},
    #[route("/reactive-hooks")]
    ReactiveHooksPage {},
    #[route("/command-palette")]
    CommandPalettePage {},
    #[route("/date-picker")]
    DatePickerPage {},
    #[route("/dropzone")]
    DropzonePage {},
    #[route("/resizable")]
    ResizablePage {},
    #[route("/carousel")]
    CarouselPage {},
    #[route("/virtual-list")]
    VirtualListPage {},
    #[route("/code-highlight")]
    CodeHighlightPage {},
    #[route("/charts")]
    ChartsPage {},
    #[route("/rich-text")]
    RichTextPage {},
    #[route("/document-card")]
    DocumentCardPage {},
    #[route("/masonry")]
    MasonryPage {},
    #[route("/media-card")]
    MediaCardPage {},
    #[route("/navigation-menu")]
    NavigationMenuPage {},
    #[route("/overflow-set")]
    OverflowSetPage {},
    #[route("/transfer-list")]
    TransferListPage {},
    #[route("/facepile")]
    FacepilePage {},
    #[route("/activity-feed")]
    ActivityFeedPage {},
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const STYLES: Asset = shadow_design::STYLES;
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let locale = use_signal(shadow_design::locale::Locale::italian);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link { rel: "stylesheet", href: STYLES }
        shadow_design::locale::LocaleProvider { locale: Some(locale),
            ThemeProvider {
                persist_key: "shadow_design_custom_theme".to_string(),
                NotificationProvider {
                    Router::<Route> {}
                }
            }
        }
    }
}

