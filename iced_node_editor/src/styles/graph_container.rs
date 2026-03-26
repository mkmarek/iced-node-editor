use iced::{Background, Color, Theme};
use palette::{Darken, FromColor, Hsl, Lighten, Srgb};

#[derive(Debug, Clone, Copy, Default)]
pub struct Appearance {
    pub background: Option<Background>,
    pub minor_guidelines_color: Option<Color>,
    pub mid_guidelines_color: Option<Color>,
    pub major_guidelines_color: Option<Color>,
    pub minor_guidelines_spacing: Option<f32>,
    pub mid_guidelines_spacing: Option<f32>,
    pub major_guidelines_spacing: Option<f32>,
}

pub trait StyleSheet {
    type Style;
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

        let base_color = palette.background.base.color;
        let base_color_srgb = Srgb::new(base_color.r, base_color.g, base_color.b);

        let text_color = palette.background.base.text;
        let text_color_srgb = Srgb::new(text_color.r, text_color.g, text_color.b);

        let base_hsl = Hsl::from_color(base_color_srgb);
        let base_text_hsl = Hsl::from_color(text_color_srgb);

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

        let minor_guidelines_color_srgb = Srgb::from_color(minor_guidelines_color);
        let mid_guidelines_color_srgb = Srgb::from_color(mid_guidelines_color);
        let major_guidelines_color_srgb = Srgb::from_color(major_guidelines_color);

        let minor_guidelines_color = Color::from_rgb(
            minor_guidelines_color_srgb.red,
            minor_guidelines_color_srgb.green,
            minor_guidelines_color_srgb.blue,
        );

        let mid_guidelines_color = Color::from_rgb(
            mid_guidelines_color_srgb.red,
            mid_guidelines_color_srgb.green,
            mid_guidelines_color_srgb.blue,
        );

        let major_guidelines_color = Color::from_rgb(
            major_guidelines_color_srgb.red,
            major_guidelines_color_srgb.green,
            major_guidelines_color_srgb.blue,
        );

        match style {
            GraphContainer::Default => Appearance {
                background: Some(Background::Color(palette.background.base.color)),
                minor_guidelines_color: Some(minor_guidelines_color),
                mid_guidelines_color: Some(mid_guidelines_color),
                major_guidelines_color: Some(major_guidelines_color),
                minor_guidelines_spacing: Some(10.0),
                mid_guidelines_spacing: Some(50.0),
                major_guidelines_spacing: Some(100.0),
            },
            GraphContainer::Custom(custom) => custom.appearance(self),
        }
    }
}
