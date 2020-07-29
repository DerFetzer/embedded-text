//! Pixel iterators used for text rendering.

pub mod character;
pub mod cursor;
pub mod line;
pub mod whitespace;

use crate::{
    alignment::TextAlignment,
    parser::Parser,
    style::{StyledTextBox, TextBoxStyle},
};
use embedded_graphics::prelude::*;

/// This trait is used to associate a state type to a horizontal alignment option.
///
/// Implementing this trait is only necessary when creating new alignment algorithms.
pub trait StateFactory {
    /// The type of the state variable used for rendering.
    type PixelIteratorState;

    /// Creates a new state variable.
    fn create_state(&self) -> Self::PixelIteratorState;
}

/// Pixel iterator for styled text.
pub struct StyledTextBoxIterator<'a, C, F, A>
where
    C: PixelColor,
    F: Font + Copy,
    A: TextAlignment,
    StyledTextBox<'a, C, F, A>: StateFactory,
{
    /// Parser to process the text during rendering.
    pub parser: Parser<'a>,

    /// Style used for rendering.
    pub style: TextBoxStyle<C, F, A>,

    /// State information used by the rendering algorithms.
    pub state: <StyledTextBox<'a, C, F, A> as StateFactory>::PixelIteratorState,
}

impl<'a, C, F, A> StyledTextBoxIterator<'a, C, F, A>
where
    C: PixelColor,
    F: Font + Copy,
    A: TextAlignment,
    StyledTextBox<'a, C, F, A>: StateFactory,
{
    /// Creates a new pixel iterator to render the styled [`TextBox`].
    ///
    /// [`TextBox`]: ../struct.TextBox.html
    #[inline]
    #[must_use]
    pub fn new(styled: &'a StyledTextBox<'a, C, F, A>) -> Self {
        Self {
            parser: Parser::parse(styled.text_box.text),
            style: styled.style,
            state: styled.create_state(),
        }
    }
}
