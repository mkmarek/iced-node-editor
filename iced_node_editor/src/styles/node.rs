use iced::{Background, Color, Theme};

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            text_color: None,
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }
}

pub trait StyleSheet {
    type Style: Default;
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

#[derive(Default)]
pub enum Node {
    #[default]
    Default,
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl StyleSheet for Theme {
    type Style = Node;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        match style {
            Node::Default => Appearance {
                background: Some(Background::Color(palette.background.base.color)),
                border_color: palette.primary.base.color,
                border_radius: 5.0,
                border_width: 1.0,
                text_color: Some(palette.primary.base.color),
            },
            Node::Custom(custom) => custom.appearance(self),
        }
    }
}
