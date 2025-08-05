use std::fmt::Display;
use std::ops::{BitAnd, BitOr, BitXor};

use crate::frame::{Cmd, Printable};
use crate::{Component, Frame};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Color {
    /// Resets the terminal color.
    Reset,

    /// Black color.
    Black,

    /// Dark grey color.
    DarkGrey,

    /// Light red color.
    Red,

    /// Dark red color.
    DarkRed,

    /// Light green color.
    Green,

    /// Dark green color.
    DarkGreen,

    /// Light yellow color.
    Yellow,

    /// Dark yellow color.
    DarkYellow,

    /// Light blue color.
    Blue,

    /// Dark blue color.
    DarkBlue,

    /// Light magenta color.
    Magenta,

    /// Dark magenta color.
    DarkMagenta,

    /// Light cyan color.
    Cyan,

    /// Dark cyan color.
    DarkCyan,

    /// White color.
    White,

    /// Grey color.
    Grey,

    /// An RGB color. See [RGB color model](https://en.wikipedia.org/wiki/RGB_color_model) for more info.
    ///
    /// Most UNIX terminals and Windows 10 supported only.
    /// See [Platform-specific notes](enum.Color.html#platform-specific-notes) for more info.
    Rgb { r: u8, g: u8, b: u8 },

    /// An ANSI color. See [256 colors - cheat sheet](https://jonasjacek.github.io/colors/) for more info.
    ///
    /// Most UNIX terminals and Windows 10 supported only.
    /// See [Platform-specific notes](enum.Color.html#platform-specific-notes) for more info.
    AnsiValue(u8),
}

/// a bitset for all possible attributes
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Attributes(u32);

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        Self(attribute.bytes())
    }
}

impl From<&[Attribute]> for Attributes {
    fn from(arr: &[Attribute]) -> Self {
        let mut attributes = Attributes::default();
        for &attr in arr {
            attributes.set(attr);
        }
        attributes
    }
}

impl BitAnd<Attribute> for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Attribute) -> Self {
        Self(self.0 & rhs.bytes())
    }
}
impl BitAnd for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitOr<Attribute> for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Attribute) -> Self {
        Self(self.0 | rhs.bytes())
    }
}
impl BitOr for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl BitXor<Attribute> for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Attribute) -> Self {
        Self(self.0 ^ rhs.bytes())
    }
}
impl BitXor for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl Attributes {
    /// Returns the empty bitset.
    #[inline(always)]
    pub const fn none() -> Self {
        Self(0)
    }

    /// Returns a copy of the bitset with the given attribute set.
    /// If it's already set, this returns the bitset unmodified.
    #[inline(always)]
    pub const fn with(self, attribute: Attribute) -> Self {
        Self(self.0 | attribute.bytes())
    }

    /// Returns a copy of the bitset with the given attribute unset.
    /// If it's not set, this returns the bitset unmodified.
    #[inline(always)]
    pub const fn without(self, attribute: Attribute) -> Self {
        Self(self.0 & !attribute.bytes())
    }

    /// Sets the attribute.
    /// If it's already set, this does nothing.
    #[inline(always)]
    pub fn set(&mut self, attribute: Attribute) {
        self.0 |= attribute.bytes();
    }

    /// Unsets the attribute.
    /// If it's not set, this changes nothing.
    #[inline(always)]
    pub fn unset(&mut self, attribute: Attribute) {
        self.0 &= !attribute.bytes();
    }

    /// Sets the attribute if it's unset, unset it
    /// if it is set.
    #[inline(always)]
    pub fn toggle(&mut self, attribute: Attribute) {
        self.0 ^= attribute.bytes();
    }

    /// Returns whether the attribute is set.
    #[inline(always)]
    pub const fn has(self, attribute: Attribute) -> bool {
        self.0 & attribute.bytes() != 0
    }

    /// Sets all the passed attributes. Removes none.
    #[inline(always)]
    pub fn extend(&mut self, attributes: Attributes) {
        self.0 |= attributes.0;
    }

    /// Returns whether there is no attribute set.
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// The style that can be put on content.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct ContentStyle {
    /// The foreground color.
    pub foreground_color: Option<Color>,
    /// The background color.
    pub background_color: Option<Color>,
    /// The underline color.
    pub underline_color: Option<Color>,
    /// List of attributes.
    pub attributes: Attributes,
}

impl ContentStyle {
    /// Creates a `StyledContent` by applying the style to the given `val`.
    #[inline]
    pub fn apply<D: Display + Clone>(self, val: D) -> StyledContent<D> {
        StyledContent::new(self, val)
    }

    /// Creates a new `ContentStyle`.
    #[inline]
    pub fn new() -> ContentStyle {
        ContentStyle::default()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StyledContent<D: Display + Clone> {
    /// The style (colors, content attributes).
    pub style: ContentStyle,
    /// A content to apply the style on.
    pub content: D,
}

impl<D: Display + Clone> AsMut<ContentStyle> for StyledContent<D> {
    fn as_mut(&mut self) -> &mut ContentStyle {
        &mut self.style
    }
}

impl<D: Display + Clone> AsRef<ContentStyle> for StyledContent<D> {
    fn as_ref(&self) -> &ContentStyle {
        &self.style
    }
}

impl<D: Display + Clone> StyledContent<D> {
    /// Creates a new `StyledContent`.
    #[inline]
    pub fn new(style: ContentStyle, content: D) -> StyledContent<D> {
        StyledContent { style, content }
    }

    /// Returns the content.
    #[inline]
    pub fn content(&self) -> &D {
        &self.content
    }

    /// Returns the style.
    #[inline]
    pub fn style(&self) -> &ContentStyle {
        &self.style
    }

    /// Returns a mutable reference to the style, so that it can be further
    /// manipulated
    #[inline]
    pub fn style_mut(&mut self) -> &mut ContentStyle {
        &mut self.style
    }

    pub fn attribute(mut self, attr: Attribute) -> Self {
        self.style_mut().attributes.set(attr);
        self
    }

    pub fn with_content<T: Display + Clone>(self, content: T) -> StyledContent<T> {
        StyledContent {
            style: self.style,
            content,
        }
    }
}

impl From<&str> for StyledContent<String> {
    fn from(s: &str) -> Self {
        StyledContent::new(ContentStyle::new(), s.to_string())
    }
}

impl From<String> for StyledContent<String> {
    fn from(s: String) -> Self {
        StyledContent::new(ContentStyle::new(), s)
    }
}

impl From<StyledContent<&str>> for StyledContent<String> {
    fn from(s: StyledContent<&str>) -> Self {
        StyledContent::new(s.style, s.content.to_string())
    }
}

impl From<StyledContent<char>> for StyledContent<String> {
    fn from(s: StyledContent<char>) -> Self {
        StyledContent::new(s.style, s.content.to_string())
    }
}

impl From<&char> for StyledContent<String> {
    fn from(c: &char) -> Self {
        StyledContent::new(ContentStyle::new(), c.to_string())
    }
}

// #[cfg(feature = "crossterm")]
// impl From<Color> for crossterm::style::Color {
//     fn from(color: Color) -> Self {
//         match color {
//             Color::Reset => Self::Reset,
//             Color::Black => Self::Black,
//             Color::DarkGrey => Self::DarkGrey,
//             Color::Red => Self::Red,
//             Color::DarkRed => Self::DarkRed,
//             Color::Green => Self::Green,
//             Color::DarkGreen => Self::DarkGreen,
//             Color::Yellow => Self::Yellow,
//             Color::DarkYellow => Self::DarkYellow,
//             Color::Blue => Self::Blue,
//             Color::DarkBlue => Self::DarkBlue,
//             Color::Magenta => Self::Magenta,
//             Color::DarkMagenta => Self::DarkMagenta,
//             Color::Cyan => Self::Cyan,
//             Color::DarkCyan => Self::DarkCyan,
//             Color::White => Self::White,
//             Color::Grey => Self::Grey,
//             Color::Rgb { r, g, b } => Self::Rgb {
//                 r: r as u8,
//                 g: g as u8,
//                 b: b as u8,
//             },
//             Color::AnsiValue(value) => Self::AnsiValue(value as u8),
//         }
//     }
// }

// #[cfg(feature = "crossterm")]
// impl From<Attributes> for crossterm::style::Attributes {
//     fn from(attributes: Attributes) -> Self {
//         let mut attrs = Self::none();
//         if attributes.has(Attribute::Underlined) {
//             attrs.set(crossterm::style::Attribute::Underlined);
//         }
//         attrs
//     }
// }

// #[cfg(feature = "crossterm")]
// impl From<ContentStyle> for crossterm::style::ContentStyle {
//     fn from(content_style: ContentStyle) -> Self {
//         Self {
//             foreground_color: content_style.foreground_color.map(|c| c.into()),
//             background_color: content_style.background_color.map(|c| c.into()),
//             underline_color: content_style.underline_color.map(|c| c.into()),
//             attributes: content_style.attributes.into(),
//         }
//     }
// }

pub fn style<D: Display + Clone>(val: D) -> StyledContent<D> {
    ContentStyle::new().apply(val)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Attribute {
    Underlined,
    Bold,
}

impl Attribute {
    /// Returns a u32 with one bit set, which is the
    /// signature of this attribute in the Attributes
    /// bitset.
    ///
    /// The +1 enables storing Reset (whose index is 0)
    ///  in the bitset Attributes.
    #[inline(always)]
    pub const fn bytes(self) -> u32 {
        1 << ((self as u32) + 1)
    }
}
pub struct Print<T>(pub T);

impl<T: Into<Printable> + Clone> Component for Print<T> {
    fn render(&self, frame: &mut Frame) -> Result<(), String> {
        frame.commands.push(Cmd::Print(self.0.clone().into()));
        Ok(())
    }
}

// #[cfg(feature = "crossterm")]
// impl<T: Display + Clone> crossterm::Command for Print<T> {
//     fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
//         crossterm::style::Print(self.0.clone()).write_ansi(f)
//     }
// }

pub trait Stylize: Sized {
    /// This type with styles applied.
    type Styled: AsRef<ContentStyle> + AsMut<ContentStyle>;

    /// Styles this type.
    fn stylize(self) -> Self::Styled;

    /// Sets the foreground color.
    fn with(self, color: Color) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_mut().foreground_color = Some(color);
        styled
    }

    /// Sets the background color.
    fn on(self, color: Color) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_mut().background_color = Some(color);
        styled
    }

    /// Sets the underline color.
    fn underline(self, color: Color) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_mut().underline_color = Some(color);
        styled
    }

    /// Styles the content with the attribute.
    fn attribute(self, attr: Attribute) -> Self::Styled {
        let mut styled = self.stylize();
        styled.as_mut().attributes.set(attr);
        styled
    }
}

impl Stylize for String {
    type Styled = StyledContent<Self>;

    fn stylize(self) -> Self::Styled {
        StyledContent::new(ContentStyle::new(), self)
    }
}

impl Stylize for &str {
    type Styled = StyledContent<Self>;

    #[inline]
    fn stylize(self) -> Self::Styled {
        StyledContent::new(ContentStyle::new(), self)
    }
}

impl Stylize for char {
    type Styled = StyledContent<Self>;

    #[inline]
    fn stylize(self) -> Self::Styled {
        StyledContent::new(ContentStyle::new(), self)
    }
}

impl AsMut<ContentStyle> for ContentStyle {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsRef<ContentStyle> for ContentStyle {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Stylize for ContentStyle {
    type Styled = Self;

    #[inline]
    fn stylize(self) -> Self::Styled {
        self
    }
}

impl<D: Display + Clone> Stylize for StyledContent<D> {
    type Styled = StyledContent<D>;
    fn stylize(self) -> Self::Styled {
        self
    }
}
