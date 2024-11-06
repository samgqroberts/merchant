use std::fmt::{self, Display};

use crossterm::{
    cursor::MoveTo,
    style::{Print, StyledContent},
    Command,
};

use crate::{comp, components::FRAME_WIDTH};

pub struct ScreenCenteredText<'a, T: Display> {
    content: &'a [T],
    content_len: usize,
    line: u16,
}

impl<'a> ScreenCenteredText<'a, String> {
    pub fn new(content: &'a [String], line: u16) -> Self {
        let content_len = content.iter().map(|x| x.len()).sum();
        Self {
            content_len,
            content,
            line,
        }
    }
}

impl<'a> ScreenCenteredText<'a, StyledContent<&'a str>> {
    pub fn new_styleds(content: &'a [StyledContent<&'a str>], line: u16) -> Self {
        let content_len = content.iter().map(|x| x.content().len()).sum();
        Self {
            content,
            content_len,
            line,
        }
    }
}

impl<'a, T: Display> Command for ScreenCenteredText<'a, T> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let content_len = self.content_len as u16;
        let starting_index =
            ((FRAME_WIDTH as f64 / 2f64) - ((content_len as f64) / 2f64)).round() as u16;
        comp!(f, MoveTo(starting_index, self.line));
        for content in self.content {
            comp!(f, Print(content));
        }
        Ok(())
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> std::io::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test::render_component;

    use super::*;

    #[test]
    fn basic() {
        let s = "012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678".to_owned();
        assert_eq!(s.len(), FRAME_WIDTH as usize);
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), " 01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), " 0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "  012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "  01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "   0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "   012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "    01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "    0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "     012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "     01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678");
        let s = s[..s.len().saturating_sub(1)].to_owned();
        assert_eq!(render_component(ScreenCenteredText::new(&[s.clone()], 0)), "      0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567");
    }
}
