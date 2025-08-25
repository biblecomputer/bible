use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub id: String,
    pub colors: ThemeColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: String,
    pub text: TextColors,
    pub verses: VerseColors,
    pub sidebar: SidebarColors,
    pub buttons: ButtonColors,
    pub header: HeaderColors,
    pub navigation: NavigationColors,
    #[serde(rename = "commandPalette")]
    pub command_palette: CommandPaletteColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextColors {
    pub primary: String,
    pub secondary: String,
    pub muted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerseColors {
    pub number: String,
    #[serde(rename = "numberHighlighted")]
    pub number_highlighted: String,
    #[serde(rename = "textHighlighted")]
    pub text_highlighted: String,
    #[serde(rename = "backgroundHighlighted")]
    pub background_highlighted: String,
    pub selected: String,
    #[serde(rename = "selectedBackground")]
    pub selected_background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarColors {
    pub background: String,
    pub border: String,
    pub text: String,
    #[serde(rename = "textHover")]
    pub text_hover: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderColors {
    pub background: String,
    pub border: String,
    pub button: HeaderButtonColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderButtonColors {
    pub text: String,
    pub hover: String,
    #[serde(rename = "hoverBackground")]
    pub hover_background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationColors {
    pub text: String,
    pub hover: String,
    #[serde(rename = "hoverBackground")]
    pub hover_background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonColors {
    pub primary: ButtonVariant,
    pub secondary: ButtonVariant,
    pub success: ButtonVariant,
    pub danger: ButtonVariant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonVariant {
    pub background: String,
    pub text: String,
    pub hover: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandPaletteColors {
    pub background: String,
    pub border: String,
    pub text: String,
    #[serde(rename = "textMuted")]
    pub text_muted: String,
    pub highlight: String,
    #[serde(rename = "highlightBackground")]
    pub highlight_background: String,
}

// Static theme loading (compile-time)
pub fn get_themes() -> Vec<Theme> {
    vec![
        serde_json::from_str(include_str!("light.json")).expect("Failed to parse light theme"),
        serde_json::from_str(include_str!("dark.json")).expect("Failed to parse dark theme"),
        serde_json::from_str(include_str!("sepia.json")).expect("Failed to parse sepia theme"),
        serde_json::from_str(include_str!("cherry_blossom.json"))
            .expect("Failed to parse cherry_blossom theme"),
        serde_json::from_str(include_str!("rose_gold.json"))
            .expect("Failed to parse rose_gold theme"),
        serde_json::from_str(include_str!("lavender_dreams.json"))
            .expect("Failed to parse lavender_dreams theme"),
        serde_json::from_str(include_str!("deuteranopia_safe.json"))
            .expect("Failed to parse deuteranopia_safe theme"),
        serde_json::from_str(include_str!("protanopia_safe.json"))
            .expect("Failed to parse protanopia_safe theme"),
        serde_json::from_str(include_str!("tritanopia_safe.json"))
            .expect("Failed to parse tritanopia_safe theme"),
        serde_json::from_str(include_str!("matrix.json")).expect("Failed to parse matrix theme"),
        serde_json::from_str(include_str!("dracula.json")).expect("Failed to parse dracula theme"),
        serde_json::from_str(include_str!("nord.json")).expect("Failed to parse nord theme"),
        serde_json::from_str(include_str!("monokai.json")).expect("Failed to parse monokai theme"),
        serde_json::from_str(include_str!("cotton_candy.json"))
            .expect("Failed to parse cotton_candy theme"),
        serde_json::from_str(include_str!("mint_cream.json"))
            .expect("Failed to parse mint_cream theme"),
        serde_json::from_str(include_str!("peach_sorbet.json"))
            .expect("Failed to parse peach_sorbet theme"),
        serde_json::from_str(include_str!("sky_blue.json"))
            .expect("Failed to parse sky_blue theme"),
        serde_json::from_str(include_str!("forest_green.json"))
            .expect("Failed to parse forest_green theme"),
        serde_json::from_str(include_str!("ocean_deep.json"))
            .expect("Failed to parse ocean_deep theme"),
        serde_json::from_str(include_str!("sunset_orange.json"))
            .expect("Failed to parse sunset_orange theme"),
        serde_json::from_str(include_str!("retro_amber.json"))
            .expect("Failed to parse retro_amber theme"),
        serde_json::from_str(include_str!("cyberpunk.json"))
            .expect("Failed to parse cyberpunk theme"),
        serde_json::from_str(include_str!("autumn_leaves.json"))
            .expect("Failed to parse autumn_leaves theme"),
        serde_json::from_str(include_str!("spring_meadow.json"))
            .expect("Failed to parse spring_meadow theme"),
        serde_json::from_str(include_str!("winter_frost.json"))
            .expect("Failed to parse winter_frost theme"),
        serde_json::from_str(include_str!("royal_purple.json"))
            .expect("Failed to parse royal_purple theme"),
        serde_json::from_str(include_str!("midnight_blue.json"))
            .expect("Failed to parse midnight_blue theme"),
        serde_json::from_str(include_str!("coral_reef.json"))
            .expect("Failed to parse coral_reef theme"),
        serde_json::from_str(include_str!("golden_hour.json"))
            .expect("Failed to parse golden_hour theme"),
        serde_json::from_str(include_str!("monochrome.json"))
            .expect("Failed to parse monochrome theme"),
        serde_json::from_str(include_str!("emerald_city.json"))
            .expect("Failed to parse emerald_city theme"),
        serde_json::from_str(include_str!("neon_nights.json"))
            .expect("Failed to parse neon_nights theme"),
        serde_json::from_str(include_str!("vintage_paper.json"))
            .expect("Failed to parse vintage_paper theme"),
        serde_json::from_str(include_str!("coffee_shop.json"))
            .expect("Failed to parse coffee_shop theme"),
        serde_json::from_str(include_str!("arctic_ice.json"))
            .expect("Failed to parse arctic_ice theme"),
        serde_json::from_str(include_str!("sunset_beach.json"))
            .expect("Failed to parse sunset_beach theme"),
        serde_json::from_str(include_str!("space_nebula.json"))
            .expect("Failed to parse space_nebula theme"),
        serde_json::from_str(include_str!("volcano_fire.json"))
            .expect("Failed to parse volcano_fire theme"),
        serde_json::from_str(include_str!("candy_pink.json"))
            .expect("Failed to parse candy_pink theme"),
        serde_json::from_str(include_str!("terminal_green.json"))
            .expect("Failed to parse terminal_green theme"),
        serde_json::from_str(include_str!("deep_ocean.json"))
            .expect("Failed to parse deep_ocean theme"),
        serde_json::from_str(include_str!("desert_sand.json"))
            .expect("Failed to parse desert_sand theme"),
        serde_json::from_str(include_str!("midnight_purple.json"))
            .expect("Failed to parse midnight_purple theme"),
        serde_json::from_str(include_str!("electric_blue.json"))
            .expect("Failed to parse electric_blue theme"),
        serde_json::from_str(include_str!("forest_night.json"))
            .expect("Failed to parse forest_night theme"),
        serde_json::from_str(include_str!("sunshine_yellow.json"))
            .expect("Failed to parse sunshine_yellow theme"),
        serde_json::from_str(include_str!("bubblegum_pop.json"))
            .expect("Failed to parse bubblegum_pop theme"),
        serde_json::from_str(include_str!("steel_gray.json"))
            .expect("Failed to parse steel_gray theme"),
        serde_json::from_str(include_str!("lime_twist.json"))
            .expect("Failed to parse lime_twist theme"),
        serde_json::from_str(include_str!("cosmic_purple.json"))
            .expect("Failed to parse cosmic_purple theme"),
        serde_json::from_str(include_str!("summer_breeze.json"))
            .expect("Failed to parse summer_breeze theme"),
        serde_json::from_str(include_str!("wine_red.json"))
            .expect("Failed to parse wine_red theme"),
        serde_json::from_str(include_str!("glacier_blue.json"))
            .expect("Failed to parse glacier_blue theme"),
        serde_json::from_str(include_str!("tropical_sunset.json"))
            .expect("Failed to parse tropical_sunset theme"),
        serde_json::from_str(include_str!("charcoal_night.json"))
            .expect("Failed to parse charcoal_night theme"),
        serde_json::from_str(include_str!("teal_wave.json"))
            .expect("Failed to parse teal_wave theme"),
        serde_json::from_str(include_str!("amber_glow.json"))
            .expect("Failed to parse amber_glow theme"),
        serde_json::from_str(include_str!("midnight_teal.json"))
            .expect("Failed to parse midnight_teal theme"),
        serde_json::from_str(include_str!("raspberry_cream.json"))
            .expect("Failed to parse raspberry_cream theme"),
    ]
}

pub fn get_theme_by_id(id: &str) -> Option<Theme> {
    get_themes().into_iter().find(|theme| theme.id == id)
}

pub fn get_default_theme() -> Theme {
    get_theme_by_id("light").expect("Default light theme not found")
}

// Helper function to convert hex colors to CSS custom properties
pub fn theme_to_css_vars(theme: &Theme) -> String {
    format!(
        r#"
        --theme-background: {};
        --theme-text-primary: {};
        --theme-text-secondary: {};
        --theme-text-muted: {};
        --theme-verse-number: {};
        --theme-verse-number-highlighted: {};
        --theme-verse-text-highlighted: {};
        --theme-verse-background-highlighted: {};
        --theme-verse-selected: {};
        --theme-verse-selected-background: {};
        --theme-sidebar-background: {};
        --theme-sidebar-border: {};
        --theme-sidebar-text: {};
        --theme-sidebar-text-hover: {};
        --theme-button-primary-background: {};
        --theme-button-primary-text: {};
        --theme-button-primary-hover: {};
        --theme-button-secondary-background: {};
        --theme-button-secondary-text: {};
        --theme-button-secondary-hover: {};
        --theme-button-success-background: {};
        --theme-button-success-text: {};
        --theme-button-success-hover: {};
        --theme-button-danger-background: {};
        --theme-button-danger-text: {};
        --theme-button-danger-hover: {};
        --theme-header-background: {};
        --theme-header-border: {};
        --theme-header-button-text: {};
        --theme-header-button-hover: {};
        --theme-header-button-hover-background: {};
        --theme-navigation-text: {};
        --theme-navigation-hover: {};
        --theme-navigation-hover-background: {};
        --theme-palette-background: {};
        --theme-palette-border: {};
        --theme-palette-text: {};
        --theme-palette-text-muted: {};
        --theme-palette-highlight: {};
        --theme-palette-highlight-background: {};
        "#,
        theme.colors.background,
        theme.colors.text.primary,
        theme.colors.text.secondary,
        theme.colors.text.muted,
        theme.colors.verses.number,
        theme.colors.verses.number_highlighted,
        theme.colors.verses.text_highlighted,
        theme.colors.verses.background_highlighted,
        theme.colors.verses.selected,
        theme.colors.verses.selected_background,
        theme.colors.sidebar.background,
        theme.colors.sidebar.border,
        theme.colors.sidebar.text,
        theme.colors.sidebar.text_hover,
        theme.colors.buttons.primary.background,
        theme.colors.buttons.primary.text,
        theme.colors.buttons.primary.hover,
        theme.colors.buttons.secondary.background,
        theme.colors.buttons.secondary.text,
        theme.colors.buttons.secondary.hover,
        theme.colors.buttons.success.background,
        theme.colors.buttons.success.text,
        theme.colors.buttons.success.hover,
        theme.colors.buttons.danger.background,
        theme.colors.buttons.danger.text,
        theme.colors.buttons.danger.hover,
        theme.colors.header.background,
        theme.colors.header.border,
        theme.colors.header.button.text,
        theme.colors.header.button.hover,
        theme.colors.header.button.hover_background,
        theme.colors.navigation.text,
        theme.colors.navigation.hover,
        theme.colors.navigation.hover_background,
        theme.colors.command_palette.background,
        theme.colors.command_palette.border,
        theme.colors.command_palette.text,
        theme.colors.command_palette.text_muted,
        theme.colors.command_palette.highlight,
        theme.colors.command_palette.highlight_background,
    )
}
