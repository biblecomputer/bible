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