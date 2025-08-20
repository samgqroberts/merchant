/// This method takes an HTML string that could contain tags like <div> and <span>
/// and parses it into a string of raw text.
/// <br /> tags are converted to newlines.
/// <span> tags are ignored (as in, classes / styling is ignored)
/// HTML entities like &nbsp; are converted to their character equivalents
pub fn parse_html_to_raw_text(html: &str) -> String {
    let mut result = String::new();
    let mut chars = html.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '<' {
            // Save position in case this isn't a valid tag
            let mut tag = String::new();
            let mut found_closing = false;
            
            // Collect the tag content
            while let Some(tag_ch) = chars.next() {
                if tag_ch == '>' {
                    found_closing = true;
                    break;
                }
                tag.push(tag_ch);
            }
            
            if !found_closing {
                // Not a complete tag, restore and treat as regular text
                result.push('<');
                result.push_str(&tag);
            } else {
                // Check if it's a br tag - handle various formats
                let normalized_tag = tag.trim().to_lowercase();
                if normalized_tag == "br" || 
                   normalized_tag == "br/" || 
                   normalized_tag == "br /" ||
                   normalized_tag.starts_with("br ") ||
                   normalized_tag.starts_with("br/") {
                    result.push('\n');
                }
                // Other tags are just ignored (their content will be processed)
            }
        } else if ch == '&' {
            // We might be starting an HTML entity
            let mut entity = String::new();
            let mut found_semicolon = false;
            
            // Peek ahead to see if this looks like an entity
            let saved_position = chars.clone();
            
            for _ in 0..10 {  // Entities shouldn't be longer than this
                if let Some(entity_ch) = chars.next() {
                    if entity_ch == ';' {
                        found_semicolon = true;
                        break;
                    } else if entity_ch.is_alphanumeric() || entity_ch == '#' {
                        entity.push(entity_ch);
                    } else {
                        // Not an entity, restore position and break
                        break;
                    }
                }
            }
            
            if found_semicolon {
                // Try to decode the entity
                let decoded = match entity.as_str() {
                    "nbsp" => ' ',
                    "lt" => '<',
                    "gt" => '>',
                    "amp" => '&',
                    "quot" => '"',
                    "apos" | "#39" => '\'',
                    _ => {
                        // Unknown entity, just output it as-is
                        result.push('&');
                        result.push_str(&entity);
                        result.push(';');
                        continue;
                    }
                };
                result.push(decoded);
            } else {
                // Not an entity, restore position and just output the &
                chars = saved_position;
                result.push('&');
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_span() {
        assert_eq!(parse_html_to_raw_text("<span>Hello</span>"), "Hello");
    }

    #[test]
    fn test_nested_spans() {
        assert_eq!(
            parse_html_to_raw_text("<span>Hello <span>World</span></span>"),
            "Hello World"
        );
    }

    #[test]
    fn test_br_tags() {
        assert_eq!(parse_html_to_raw_text("Hello<br />World"), "Hello\nWorld");
        assert_eq!(parse_html_to_raw_text("Hello<br/>World"), "Hello\nWorld");
        assert_eq!(parse_html_to_raw_text("Hello<br>World"), "Hello\nWorld");
        assert_eq!(parse_html_to_raw_text("Hello<BR />World"), "Hello\nWorld");
    }

    #[test]
    fn test_html_entities() {
        assert_eq!(parse_html_to_raw_text("Hello&nbsp;World"), "Hello World");
        assert_eq!(parse_html_to_raw_text("&lt;script&gt;"), "<script>");
        assert_eq!(parse_html_to_raw_text("&amp;&amp;"), "&&");
        assert_eq!(parse_html_to_raw_text("&quot;quoted&quot;"), "\"quoted\"");
        assert_eq!(parse_html_to_raw_text("it&#39;s"), "it's");
        assert_eq!(parse_html_to_raw_text("it&apos;s"), "it's");
    }

    #[test]
    fn test_mixed_content() {
        assert_eq!(
            parse_html_to_raw_text("<span class=\"bold\">Bold</span>&nbsp;text"),
            "Bold text"
        );
        assert_eq!(
            parse_html_to_raw_text("<span style=\"color: red\">Red</span><br /><span>Normal</span>"),
            "Red\nNormal"
        );
    }

    #[test]
    fn test_no_html() {
        assert_eq!(parse_html_to_raw_text("Plain text"), "Plain text");
        assert_eq!(parse_html_to_raw_text(""), "");
    }

    #[test]
    fn test_incomplete_tags() {
        assert_eq!(parse_html_to_raw_text("Hello < World"), "Hello < World");
        assert_eq!(parse_html_to_raw_text("Hello & World"), "Hello & World");
    }

    #[test]
    fn test_complex_html() {
        let html = "<span class=\"bold underline\" style=\"color: red\">Styled</span>";
        assert_eq!(parse_html_to_raw_text(html), "Styled");
        
        let html2 = "Normal&nbsp;<span class=\"bold\">Bold</span>&nbsp;<span class=\"underline\">Underline</span>";
        assert_eq!(parse_html_to_raw_text(html2), "Normal Bold Underline");
    }

    #[test]
    fn test_multiple_br_tags() {
        assert_eq!(
            parse_html_to_raw_text("<br /><br />&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;X"),
            "\n\n     X"
        );
    }

    #[test]
    fn test_html_renderer_output() {
        // Test the actual output format from HtmlRenderer
        let html = "Hello,&nbsp;World!<br />";
        assert_eq!(parse_html_to_raw_text(html), "Hello, World!\n");
        
        let html2 = "<span style=\"color: red\">Red</span>&nbsp;<span style=\"color: #00f\">Blue</span>";
        assert_eq!(parse_html_to_raw_text(html2), "Red Blue");
    }

    #[test]
    fn test_escaped_html_in_content() {
        let html = "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;";
        assert_eq!(parse_html_to_raw_text(html), "<script>alert('xss')</script>");
    }

    #[test]
    fn test_div_tags() {
        assert_eq!(parse_html_to_raw_text("<div>Content</div>"), "Content");
        assert_eq!(
            parse_html_to_raw_text("<div>Line1</div><div>Line2</div>"),
            "Line1Line2"
        );
    }

    #[test]
    fn test_self_closing_tags() {
        assert_eq!(parse_html_to_raw_text("Before<br/>After"), "Before\nAfter");
        assert_eq!(parse_html_to_raw_text("Before<img />After"), "BeforeAfter");
    }
}