use terminal_commands::frame::{Frame, Printable, RenderOutput, RenderResult, Renderer};
use terminal_commands::style::{Attribute, Color, ContentStyle, StyledContent};

pub struct HtmlRenderer;

impl Renderer for HtmlRenderer {
    fn render(&self, frame: &Frame) -> RenderResult {
        let mut chars: Vec<Vec<StyledContent<char>>> = vec![vec![]];
        let mut cursor_x = 0;
        let mut cursor_y = 0;
        let mut show_cursor = false;

        for cmd in frame.commands().iter() {
            match cmd {
                terminal_commands::frame::Cmd::ClearScreen => {
                    chars.clear();
                    chars.push(Vec::new());
                    cursor_x = 0;
                    cursor_y = 0;
                }
                terminal_commands::frame::Cmd::MoveTo(x, y) => {
                    cursor_x = 0;
                    while cursor_y < *y {
                        cursor_y += 1;
                        while chars.len() <= cursor_y.into() {
                            chars.push(Vec::new());
                        }
                    }
                    cursor_y = *y;
                    while cursor_x < *x {
                        cursor_x += 1;
                        let line = &mut chars[Into::<usize>::into(cursor_y)];
                        while line.len() < cursor_x.into() {
                            line.push(StyledContent::new(ContentStyle::new(), ' '));
                        }
                    }
                }
                terminal_commands::frame::Cmd::MoveUp(y) => {
                    cursor_y = cursor_y.saturating_sub(*y);
                }
                terminal_commands::frame::Cmd::MoveDown(y) => {
                    cursor_y += y;
                    while cursor_y >= chars.len() as u16 {
                        chars.push(Vec::new());
                    }
                }
                terminal_commands::frame::Cmd::MoveLeft(x) => {
                    cursor_x = cursor_x.saturating_sub(*x);
                }
                terminal_commands::frame::Cmd::MoveRight(x) => {
                    cursor_x += x;
                }
                terminal_commands::frame::Cmd::HideCursor => {
                    show_cursor = false;
                }
                terminal_commands::frame::Cmd::ShowCursor => {
                    show_cursor = true;
                }
                terminal_commands::frame::Cmd::MoveToNextLine(y) => {
                    for _ in 0..*y {
                        if chars.len() <= cursor_y.into() {
                            chars.push(Vec::new());
                        }
                        cursor_y += 1;
                    }
                    cursor_x = 0;
                }
                terminal_commands::frame::Cmd::Print(printable) => {
                    let content = printable.raw_text();
                    let style = match printable {
                        Printable::String(_) => ContentStyle::new(),
                        Printable::Char(_) => ContentStyle::new(),
                        Printable::StyledContent(styled_content) => styled_content.style().clone(),
                    };
                    for char in content.chars() {
                        let line = &mut chars[Into::<usize>::into(cursor_y)];
                        while line.len() <= cursor_x.into() {
                            line.push(StyledContent::new(ContentStyle::new(), ' '));
                        }
                        chars[Into::<usize>::into(cursor_y)][Into::<usize>::into(cursor_x)] =
                            StyledContent::new(style, char);
                        cursor_x += 1;
                    }
                }
            }
        }

        let mut html = String::new();
        for (line_num, line) in chars.iter().enumerate() {
            for char in line {
                html.push_str(&render_printable_to_html(&Printable::StyledContent(
                    (*char).into(),
                )));
            }
            if line_num < chars.len() - 1 {
                html.push_str("<br />");
            }
        }

        Ok(RenderOutput {
            result: html,
            cursor: (cursor_x, cursor_y),
            show_cursor,
        })
    }
}

fn render_printable_to_html(printable: &Printable) -> String {
    match printable {
        Printable::String(s) => escape_html(s),
        Printable::Char(c) => escape_html(&c.to_string()),
        Printable::StyledContent(styled) => {
            let mut classes = Vec::new();
            let mut inline_styles = Vec::new();

            // Handle text attributes
            if styled.style.attributes.has(Attribute::Bold) {
                classes.push("bold");
            }
            if styled.style.attributes.has(Attribute::Underlined) {
                classes.push("underline");
            }

            // Handle colors
            if let Some(fg) = &styled.style.foreground_color {
                inline_styles.push(format!("color: {}", color_to_css(*fg)));
            }
            if let Some(bg) = &styled.style.background_color {
                inline_styles.push(format!("background-color: {}", color_to_css(*bg)));
            }
            if let Some(ul) = &styled.style.underline_color {
                inline_styles.push(format!("text-decoration-color: {}", color_to_css(*ul)));
            }

            let escaped_content = escape_html(styled.content());

            if classes.is_empty() && inline_styles.is_empty() {
                escaped_content
            } else {
                let mut span_attrs = Vec::new();

                if !classes.is_empty() {
                    span_attrs.push(format!("class=\"{}\"", classes.join(" ")));
                }

                if !inline_styles.is_empty() {
                    span_attrs.push(format!("style=\"{}\"", inline_styles.join("; ")));
                }

                format!("<span {}>{}</span>", span_attrs.join(" "), escaped_content)
            }
        }
    }
}

fn escape_html(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            ' ' => "&nbsp;".to_string(),
            '\n' => "<br />".to_string(),
            c => c.to_string(),
        })
        .collect()
}

fn color_to_css(color: Color) -> String {
    match color {
        Color::Reset => "inherit".to_string(),
        Color::Black => "black".to_string(),
        Color::DarkGrey => "#555".to_string(),
        Color::Red => "red".to_string(),
        Color::DarkRed => "#800".to_string(),
        Color::Green => "lime".to_string(),
        Color::DarkGreen => "green".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::DarkYellow => "#880".to_string(),
        Color::Blue => "#00f".to_string(),
        Color::DarkBlue => "#008".to_string(),
        Color::Magenta => "magenta".to_string(),
        Color::DarkMagenta => "#808".to_string(),
        Color::Cyan => "cyan".to_string(),
        Color::DarkCyan => "#088".to_string(),
        Color::White => "white".to_string(),
        Color::Grey => "#aaa".to_string(),
        Color::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
        Color::AnsiValue(value) => format!("color-{}", value),
    }
}

#[cfg(test)]
mod tests {
    use crate::test::raw_parse_html::parse_html_to_raw_text;

    use super::*;
    use merchant_core::components::{FrameType, ScreenCenteredText};
    use merchant_core::engine::render_scene;
    use merchant_core::state::GameState;
    use merchant_core::test::rng::MockRng;
    use pretty_assertions::assert_eq;
    use terminal_commands::comp;
    use terminal_commands::cursor::MoveTo;
    use terminal_commands::frame::Frame;
    use terminal_commands::style::{Print, Stylize};

    #[test]
    fn empty_frame() {
        let renderer = HtmlRenderer;
        let frame = Frame::new();
        let result = renderer.render(&frame);
        assert_eq!(
            result.unwrap(),
            RenderOutput {
                result: "".to_string(),
                cursor: (0, 0),
                show_cursor: false,
            }
        );
    }

    #[test]
    fn hello_world() {
        let renderer = HtmlRenderer;
        let mut frame = Frame::new();
        comp!(frame, Print("Hello, World!")).unwrap();
        let result = renderer.render(&frame);
        assert_eq!(
            result.unwrap(),
            RenderOutput {
                result: "Hello,&nbsp;World!".to_string(),
                cursor: (13, 0),
                show_cursor: false,
            }
        );
    }

    #[test]
    fn test_move_to() {
        let renderer = HtmlRenderer;
        let mut frame = Frame::new();
        comp!(
            frame,
            MoveTo(5, 2),
            Print("X"),
            MoveTo(4, 1),
            Print("Y"),
            MoveTo(2, 2),
            Print("Z")
        )
        .unwrap();
        let result = renderer.render(&frame);
        assert_eq!(
            result.unwrap(),
            RenderOutput {
                result: "<br />&nbsp;&nbsp;&nbsp;&nbsp;Y<br />&nbsp;&nbsp;Z&nbsp;&nbsp;X"
                    .to_string(),
                cursor: (3, 2),
                show_cursor: false,
            }
        );
    }

    #[test]
    fn test_styled_text() {
        let renderer = HtmlRenderer;
        let mut frame = Frame::new();
        comp!(
            frame,
            Print("AB".to_string()),
            Print(" ".to_string()),
            Print("CD".to_string().bold()),
            Print(" ".to_string()),
            Print("EF".to_string().underlined())
        )
        .unwrap();
        let result = renderer.render(&frame);
        assert_eq!(
            result.unwrap(),
            RenderOutput {
                result: "AB&nbsp;<span class=\"bold\">C</span><span class=\"bold\">D</span>&nbsp;<span class=\"underline\">E</span><span class=\"underline\">F</span>".to_string(),
                cursor: (8, 0),
                show_cursor: false,
            }
        );
    }

    #[test]
    fn test_colored_text() {
        let renderer = HtmlRenderer;
        let mut frame = Frame::new();
        comp!(
            frame,
            Print("AB".to_string().red()),
            Print(" ".to_string()),
            Print("CD".to_string().blue())
        )
        .unwrap();
        let result = renderer.render(&frame);
        assert_eq!(
            result.unwrap(),
            RenderOutput {
                result: "<span style=\"color: red\">A</span><span style=\"color: red\">B</span>&nbsp;<span style=\"color: #00f\">C</span><span style=\"color: #00f\">D</span>".to_string(),
                cursor: (5, 0),
                show_cursor: false,
            }
        );
    }

    #[test]
    fn test_styled_and_colored_text() {
        let renderer = HtmlRenderer;
        let mut frame = Frame::new();
        comp!(frame, Print("AB".to_string().bold().red().underlined())).unwrap();
        let result = renderer.render(&frame);
        assert_eq!(
            result.unwrap(),
            RenderOutput {
                result: "<span class=\"bold underline\" style=\"color: red\">A</span><span class=\"bold underline\" style=\"color: red\">B</span>"
                    .to_string(),
                cursor: (2, 0),
                show_cursor: false,
            }
        );
    }

    #[test]
    fn test_html_escaping() {
        let renderer = HtmlRenderer;
        let mut frame = Frame::new();
        comp!(frame, Print("<script>alert('xss')</script>")).unwrap();
        let result = renderer.render(&frame);
        assert_eq!(
            result.unwrap(),
            RenderOutput {
                result: "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;".to_string(),
                cursor: (29, 0),
                show_cursor: false,
            }
        );
    }

    #[test]
    fn component_test_frame() {
        let mut frame = Frame::new();
        let component = merchant_core::components::SceneFrame(FrameType::SimpleEmptyInside);
        comp!(frame, component).unwrap();
        let result = HtmlRenderer.render(&frame).unwrap();
        assert_eq!(
            parse_html_to_raw_text(&result.result),
            r#"---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------"#,
        );
    }

    #[test]
    fn component_test_screen_centered_text() {
        let mut frame = Frame::new();
        comp!(
            frame,
            ScreenCenteredText::new(
                &[
                    "1---------2---------3---------4--------5--------6---------7---------"
                        .to_owned()
                ],
                1
            ),
            ScreenCenteredText::new(&["A tribute to Drug Wars by samgqroberts".to_owned()], 1)
        )
        .unwrap();
        let result = HtmlRenderer.render(&frame).unwrap();
        assert_eq!(
            parse_html_to_raw_text(&result.result),
            r#"
                1---------2----A tribute to Drug Wars by samgqroberts-----7---------"#,
        );
    }

    #[test]
    fn component_test_frame_plus_text() {
        let mut frame = Frame::new();
        let component = merchant_core::components::SceneFrame(FrameType::SimpleEmptyInside);
        let x = ScreenCenteredText::new(&["A tribute to Drug Wars by samgqroberts".to_owned()], 12);
        comp!(frame, component, x).unwrap();
        let result = HtmlRenderer.render(&frame).unwrap();
        assert_eq!(
            parse_html_to_raw_text(&result.result),
            r#"---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                              A tribute to Drug Wars by samgqroberts                             |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------"#,
        );
    }

    #[test]
    fn integration_test_splash_screen() {
        let (frame, _) = render_scene(&mut GameState::new(
            MockRng::new_with_default_locations().into(),
        ))
        .unwrap();
        let result = HtmlRenderer.render(&frame).unwrap();
        assert_eq!(
            parse_html_to_raw_text(&result.result),
            r"---------------------------------------------------------------------------------------------------
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                            __  __               _                 _                             |
|                           |  \/  |             | |               | |                            |
|                           | \  / | ___ _ __ ___| |__   __ _ _ __ | |_                           |
|                           | |\/| |/ _ \ '__/ __| '_ \ / _` | '_ \| __|                          |
|                           | |  | |  __/ | | (__| | | | (_| | | | | |_                           |
|                           |_|  |_|\___|_|  \___|_| |_|\__,_|_| |_|\__|                          |
|                                                                                                 |
|                                                                                                 |
|                              A tribute to Drug Wars by samgqroberts                             |
|                                                                                                 |
|                                       www.samgqroberts.com                                      |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                      Press any key to begin                                     |
|                                                                                                 |
|                                                                                                 |
|                                                                                                 |
|                                    ctrl-c to quit at any time                                   |
|                                                                                                 |
|                                                                                                 |
---------------------------------------------------------------------------------------------------",
        );
    }
}
