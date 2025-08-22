use crate::style::StyledContent;

#[derive(Debug, Clone)]
pub enum Printable {
    String(String),
    Char(char),
    StyledContent(StyledContent<String>),
}

impl Printable {
    pub fn raw_text(&self) -> String {
        match self {
            Printable::String(s) => s.clone(),
            Printable::Char(c) => c.to_string(),
            Printable::StyledContent(styled_content) => styled_content.content().to_string(),
        }
    }
}

impl From<&char> for Printable {
    fn from(c: &char) -> Self {
        Printable::Char(*c)
    }
}

impl From<String> for Printable {
    fn from(s: String) -> Self {
        Printable::String(s)
    }
}

impl From<char> for Printable {
    fn from(c: char) -> Self {
        Printable::Char(c)
    }
}

impl From<StyledContent<String>> for Printable {
    fn from(styled_content: StyledContent<String>) -> Self {
        Printable::StyledContent(styled_content)
    }
}

impl From<&str> for Printable {
    fn from(s: &str) -> Self {
        Printable::String(s.to_string())
    }
}

impl From<StyledContent<&str>> for Printable {
    fn from(styled_content: StyledContent<&str>) -> Self {
        Printable::StyledContent(styled_content.into())
    }
}
