//! Textbox style builder.
use crate::{alignment::LeftAligned, alignment::TextAlignment, style::TextBoxStyle};
use embedded_graphics::{
    prelude::*,
    style::{TextStyle, TextStyleBuilder},
};

/// Textbox style builder object.
pub struct TextBoxStyleBuilder<C, F, A>
where
    C: PixelColor,
    F: Font + Copy,
    A: TextAlignment,
{
    text_style_builder: TextStyleBuilder<C, F>,
    alignment: A,
}

impl<C, F> TextBoxStyleBuilder<C, F, LeftAligned>
where
    C: PixelColor,
    F: Font + Copy,
{
    /// Creates a new textbox style builder with a given font.
    #[inline]
    #[must_use]
    pub fn new(font: F) -> Self {
        Self {
            text_style_builder: TextStyleBuilder::new(font),
            alignment: LeftAligned,
        }
    }
}

impl<C, F, A> TextBoxStyleBuilder<C, F, A>
where
    C: PixelColor,
    F: Font + Copy,
    A: TextAlignment,
{
    /// Sets the text color.
    #[inline]
    #[must_use]
    pub fn text_color(self, text_color: C) -> Self {
        Self {
            text_style_builder: self.text_style_builder.text_color(text_color),
            ..self
        }
    }

    /// Sets the background color.
    #[inline]
    #[must_use]
    pub fn background_color(self, background_color: C) -> Self {
        Self {
            text_style_builder: self.text_style_builder.background_color(background_color),
            ..self
        }
    }

    /// Copies properties from an existing text style object.
    #[inline]
    #[must_use]
    pub fn text_style(self, text_style: TextStyle<C, F>) -> Self {
        let mut text_style_builder = self.text_style_builder;

        if let Some(color) = text_style.background_color {
            text_style_builder = text_style_builder.background_color(color);
        }

        if let Some(color) = text_style.text_color {
            text_style_builder = text_style_builder.text_color(color);
        }

        Self {
            text_style_builder,
            ..self
        }
    }

    /// Sets the text alignment.
    #[inline]
    #[must_use]
    pub fn alignment<AA: TextAlignment>(self, alignment: AA) -> TextBoxStyleBuilder<C, F, AA> {
        TextBoxStyleBuilder {
            text_style_builder: self.text_style_builder,
            alignment,
        }
    }

    /// Builds the text style.
    #[inline]
    #[must_use]
    pub fn build(self) -> TextBoxStyle<C, F, A> {
        TextBoxStyle {
            text_style: self.text_style_builder.build(),
            alignment: self.alignment,
        }
    }
}

#[cfg(test)]
mod test {
    use super::TextBoxStyleBuilder;
    use embedded_graphics::{
        fonts::Font6x8,
        pixelcolor::BinaryColor,
        style::{TextStyle, TextStyleBuilder},
    };

    #[test]
    fn test_text_style_copy() {
        let text_styles: [TextStyle<_, _>; 2] = [
            TextStyleBuilder::new(Font6x8)
                .text_color(BinaryColor::On)
                .build(),
            TextStyleBuilder::new(Font6x8)
                .background_color(BinaryColor::On)
                .build(),
        ];

        for &text_style in text_styles.iter() {
            let style = TextBoxStyleBuilder::new(Font6x8)
                .text_style(text_style)
                .build();

            assert_eq!(style.text_style, text_style);
        }
    }
}
