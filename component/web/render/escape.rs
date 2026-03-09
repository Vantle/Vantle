use std::fmt::Write;

pub fn stream(output: &mut String, text: &str) {
    for character in text.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(character),
        }
    }
}

#[must_use]
pub fn escape(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    stream(&mut output, text);
    output
}

#[must_use]
pub fn bytes(data: &[u8]) -> String {
    let mut output = String::with_capacity(data.len());
    for &b in data {
        if b.is_ascii_graphic() || b == b' ' {
            output.push(b as char);
        } else {
            write!(output, "\\x{b:02x}").unwrap();
        }
    }
    output
}
