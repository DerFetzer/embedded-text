//! Parse text into words, newlines and whitespace sequences.
//!
//! ```rust
//! use embedded_text::parser::{Parser, Token};
//!
//! let parser = Parser::parse("Hello, world!\n");
//! let tokens = parser.collect::<Vec<Token<'_>>>();
//!
//! assert_eq!(
//!     vec![
//!         Token::Word("Hello,"),
//!         Token::Whitespace(1),
//!         Token::Word("world!"),
//!         Token::NewLine
//!     ],
//!     tokens
//! );
//! ```
use core::str::Chars;

/// A text token
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    /// A newline character.
    NewLine,

    /// A \r character.
    CarriageReturn,

    /// A number of whitespace characters.
    Whitespace(u32),

    /// A word (a sequence of non-whitespace characters).
    Word(&'a str),
}

/// Text parser. Turns a string into a stream of [`Token`] objects.
///
/// [`Token`]: enum.Token.html
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    inner: Chars<'a>,
}

impl<'a> Parser<'a> {
    /// Create a new parser object to process the given piece of text.
    #[inline]
    #[must_use]
    pub fn parse(text: &'a str) -> Self {
        Self {
            inner: text.chars(),
        }
    }

    /// Returns the next token without advancing.
    #[inline]
    #[must_use]
    pub fn peek(&self) -> Option<Token> {
        self.clone().next()
    }

    /// Returns true if there are no tokens to process.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.as_str().is_empty()
    }

    /// Returns the number of unprocessed bytes.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.inner.as_str().len()
    }

    fn is_word_char(c: char) -> bool {
        (!c.is_whitespace() || c == '\u{A0}') && c != '\u{200B}'
    }

    fn is_space_char(c: char) -> bool {
        // '\u{200B}' (zero-width space) breaks whitespace sequences - this works as long as
        // space handling is symmetrical (i.e. starting == ending behaviour)
        c.is_whitespace() && !['\n', '\r', '\u{A0}', '\u{200B}'].contains(&c) || c == '\u{200B}'
    }

    fn is_breaking_char(c: char) -> bool {
        c.is_ascii_punctuation()
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let token = 'parse: loop {
            let string = self.inner.as_str();
            if let Some(c) = self.inner.next() {
                let mut iter = self.inner.clone();

                if Self::is_word_char(c) {
                    if Self::is_breaking_char(c) {
                        break 'parse Token::Word(unsafe {
                            // don't worry
                            string.get_unchecked(0..1)
                        });
                    }

                    while let Some(c) = iter.next() {
                        let end_token = if Self::is_word_char(c) {
                            self.inner = iter.clone();
                            Self::is_breaking_char(c)
                        } else {
                            true
                        };

                        if end_token {
                            let offset = string.len() - self.inner.as_str().len();
                            break 'parse Token::Word(unsafe {
                                // don't worry
                                string.get_unchecked(0..offset)
                            });
                        }
                    }

                    // consume all the text
                    self.inner = "".chars();
                    break Token::Word(&string);
                } else {
                    match c {
                        '\n' => break 'parse Token::NewLine,
                        '\r' => break 'parse Token::CarriageReturn,
                        '\u{200B}' => {}

                        _ => {
                            let mut len = 1;
                            while let Some(c) = iter.next() {
                                if Self::is_space_char(c) {
                                    if c != '\u{200B}' {
                                        len += 1;
                                    }
                                    self.inner = iter.clone();
                                } else {
                                    // consume the whitespaces
                                    break 'parse Token::Whitespace(len);
                                }
                            }

                            // consume all the text
                            self.inner = "".chars();
                            break Token::Whitespace(len);
                        }
                    }
                }
            } else {
                return None;
            }
        };

        Some(token)
    }
}

#[cfg(test)]
mod test {
    use super::{Parser, Token};
    #[test]
    fn parse() {
        // (At least) for now, \r is considered a whitespace
        let text = "Lorem ipsum \r dolor sit amet, conse😅ctetur adipiscing\nelit";

        assert_eq!(
            Parser::parse(text).collect::<Vec<Token>>(),
            vec![
                Token::Word("Lorem"),
                Token::Whitespace(1),
                Token::Word("ipsum"),
                Token::Whitespace(1),
                Token::CarriageReturn,
                Token::Whitespace(1),
                Token::Word("dolor"),
                Token::Whitespace(1),
                Token::Word("sit"),
                Token::Whitespace(1),
                Token::Word("amet,"),
                Token::Whitespace(1),
                Token::Word("conse😅ctetur"),
                Token::Whitespace(1),
                Token::Word("adipiscing"),
                Token::NewLine,
                Token::Word("elit"),
            ]
        );
    }

    #[test]
    fn parse_multibyte_last() {
        let text = "two\u{200B}words";
        assert_eq!(9, "two\u{200B}words".chars().count());

        assert_eq!(
            Parser::parse(text).collect::<Vec<Token>>(),
            vec![Token::Word("two"), Token::Word("words")]
        );

        assert_eq!(
            Parser::parse("  \u{200B} ").collect::<Vec<Token>>(),
            vec![Token::Whitespace(3)]
        );
    }

    #[test]
    fn parse_zwsp() {
        let text = "test😅";

        assert_eq!(
            Parser::parse(text).collect::<Vec<Token>>(),
            vec![Token::Word("test😅"),]
        );
    }

    #[test]
    fn parse_punctuation() {
        let text = "..word,word";

        assert_eq!(
            Parser::parse(text).collect::<Vec<Token>>(),
            vec![
                Token::Word("."),
                Token::Word("."),
                Token::Word("word,"),
                Token::Word("word")
            ]
        );
    }

    #[test]
    fn parse_nbsp_as_word_char() {
        let text = "test\u{A0}word";

        assert_eq!(9, "test\u{A0}word".chars().count());
        assert_eq!(
            Parser::parse(text).collect::<Vec<Token>>(),
            vec![Token::Word("test\u{A0}word"),]
        );
        assert_eq!(
            Parser::parse(" \u{A0}word").collect::<Vec<Token>>(),
            vec![Token::Whitespace(1), Token::Word("\u{A0}word"),]
        );
    }
}
