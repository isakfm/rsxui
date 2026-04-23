//! HTML utilities for the website.

/// Escape HTML special chars and braces for safe display in `<code>` blocks.
pub fn escape_code(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('{', "&#123;")
        .replace('}', "&#125;")
}

/// Generate a code example block with syntax highlighting structure.
pub fn code_example(lines: &[&str]) -> String {
    let inner: String = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let num = i + 1;
            let escaped = escape_code(line);
            format!(
                r#"<pre data-prefix="{}"><code>{}</code></pre>"#,
                num, escaped
            )
        })
        .collect();

    format!(r#"<div class="mockup-code mb-8">{}</div>"#, inner)
}
