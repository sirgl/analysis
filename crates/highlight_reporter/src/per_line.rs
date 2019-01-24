pub fn print_per_line<F: Fn(usize) -> Option<String>>(
    text: &str,
    indent: u32,
    line_callback: F,
) -> String {
    let mut sb = String::new();
    for (index, line) in text.lines().enumerate() {
        push_indent(&mut sb, indent);
        sb.push_str(line);
        sb.push('\n');
        if let Some(attached_text) = line_callback(index) {
            sb.push_str(attached_text.as_str());
        }
    }
    sb
}

pub fn push_indent(str: &mut String, count: u32) {
    for i in 0..count {
        str.push(' ');
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_indented() {
        let string = print_per_line(
            "foo\nbar\nbaz",
            2,
            |index| {
                if index == 1 || index == 2 {
                    Some(index.to_string())
                } else {
                    None
                }
            }
        );
        assert_eq!(string, "  foo\n  bar\n1  baz\n2")
    }
}