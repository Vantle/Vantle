use std::fmt::Write;

#[must_use]
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[must_use]
pub fn bytes(data: &[u8]) -> String {
    data.iter()
        .map(|b| {
            if b.is_ascii_graphic() || *b == b' ' {
                (*b as char).to_string()
            } else {
                format!("\\x{b:02x}")
            }
        })
        .collect::<String>()
}

pub fn open(output: &mut String, class: &str) {
    write!(output, "<span class=\"node-{class}\">").unwrap();
}

pub fn close(output: &mut String) {
    output.push_str("</span>");
}
