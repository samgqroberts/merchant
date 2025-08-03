use ansi_commands::{
    comp,
    cursor::MoveTo,
    frame::Printable,
    style::{Print, StyledContent},
    Component,
};

use crate::components::FRAME_WIDTH;

pub struct ScreenCenteredText {
    content: Vec<Printable>,
    content_len: usize,
    line: u16,
}

impl ScreenCenteredText {
    pub fn new(content: &[String], line: u16) -> Self {
        let content_len = content.iter().map(|x| x.len()).sum();
        Self {
            content_len,
            content: content.iter().map(|x| x.clone().into()).collect(),
            line,
        }
    }
}

impl ScreenCenteredText {
    pub fn new_styleds(content: &[StyledContent<&str>], line: u16) -> Self {
        let content_len = content.iter().map(|x| x.content().len()).sum();
        Self {
            content: content.iter().map(|x| x.clone().into()).collect(),
            content_len,
            line,
        }
    }
}

impl Component for ScreenCenteredText {
    fn render(&self, f: &mut ansi_commands::frame::Frame) -> Result<(), String> {
        let content_len = self.content_len as u16;
        let starting_index =
            ((FRAME_WIDTH as f64 / 2f64) - ((content_len as f64) / 2f64)).round() as u16;
        comp!(f, MoveTo(starting_index, self.line))?;
        for content in &self.content {
            comp!(f, Print(content.clone()))?;
        }
        Ok(())
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
