use iced::{Background, Color, Theme};
use palette::{Hsl, Srgb, FromColor, Shade};

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    pub background: Option<Background>,
    pub minor_guidelines_color: Option<Color>,
    pub mid_guidelines_color: Option<Color>,
    pub major_guidelines_color: Option<Color>,
    pub minor_guidelines_spacing: Option<f32>,
    pub mid_guidelines_spacing: Option<f32>,
    pub major_guidelines_spacing: Option<f32>,
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            background: None,
            minor_guidelines_color: None,
            mid_guidelines_color: None,
            major_guidelines_color: None,
            minor_guidelines_spacing: None,
            mid_guidelines_spacing: None,
            major_guidelines_spacing: None,
        }
    }
}

pub trait StyleSheet {
    type Style: Default;
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

#[derive(Default)]
pub enum GraphContainer {
    #[default]
    Default,
    Custom(Box<dyn StyleSheet<Style = Theme>>),
}

impl StyleSheet for Theme {
    type Style = GraphContainer;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        let base_hsl = Hsl::from_color(Srgb::from(palette.background.base.color));
        let base_text_hsl = Hsl::from_color(Srgb::from(palette.background.base.text));

        let minor_guidelines_color = if base_hsl.lightness > base_text_hsl.lightness {
            base_hsl.darken(0.02)
        } else {
            base_hsl.lighten(0.02)
        };

        let mid_guidelines_color = if base_hsl.lightness > base_text_hsl.lightness {
            base_hsl.darken(0.05)
        } else {
            base_hsl.lighten(0.04)
        };

        let major_guidelines_color = if base_hsl.lightness > base_text_hsl.lightness {
            base_hsl.darken(0.1)
        } else {
            base_hsl.lighten(0.08)
        };

        match style {
            GraphContainer::Default => Appearance {
                background: Some(Background::Color(palette.background.base.color)),
                minor_guidelines_color: Some(Srgb::from_color(minor_guidelines_color).into()),
                mid_guidelines_color: Some(Srgb::from_color(mid_guidelines_color).into()),
                major_guidelines_color: Some(Srgb::from_color(major_guidelines_color).into()),
                minor_guidelines_spacing: Some(10.0),
                mid_guidelines_spacing: Some(50.0),
                major_guidelines_spacing: Some(100.0),
            },
            GraphContainer::Custom(custom) => custom.appearance(self),
        }
    }
}
