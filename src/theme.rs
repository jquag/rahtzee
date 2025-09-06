use ratatui::style::Color;

pub struct Theme;

impl Theme {
    // Primary colors
    pub const PRIMARY: Color = Color::Indexed(147);
    pub const BORDER: Color = Color::Indexed(50);
    pub const TEXT: Color = Color::White;
    pub const TEXT_DIM: Color = Color::Indexed(241);
    pub const ACCENT: Color = Color::Indexed(213);
}
