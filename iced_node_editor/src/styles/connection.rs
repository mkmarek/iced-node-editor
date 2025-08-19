use iced::{Color, Theme};

#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
    pub color: Option<Color>,
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
                color: Some(palette.primary.base.color),
            },
            Node::Custom(custom) => custom.appearance(self),
        }
    }
}
