use std::fmt::Write;

pub fn stream(output: &mut String, text: &str) {
    let bytes = text.as_bytes();
    let mut position = 0;
    for (index, &byte) in bytes.iter().enumerate() {
        let replacement = match byte {
            b'&' => "&amp;",
            b'<' => "&lt;",
            b'>' => "&gt;",
            b'"' => "&quot;",
            b'\'' => "&#39;",
            _ => continue,
        };
        output.push_str(&text[position..index]);
        output.push_str(replacement);
        position = index + 1;
    }
    output.push_str(&text[position..]);
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
