
pub fn unescape(inner: &str) -> String {
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(esc) = chars.next() {
                match esc {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    '\'' => out.push('\''),
                    other => out.push(other),
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

pub fn parse_long_string<'a>(input: &'a str) -> Option<(&'a str, usize)> {
    let bytes = input.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'[' {
        return None;
    }
    // find second '[' after zero or more '='
    let mut i = 1;
    while i < bytes.len() && bytes[i] == b'=' {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'[' {
        return None;
    }
    let open_len = i + 1; // length of opening delimiter, e.g. "[[", "[=["
    let eqs = &input[1..i]; // the '=' sequence (may be empty)

    let mut closing = String::with_capacity(2 + eqs.len());
    closing.push(']');
    closing.push_str(eqs);
    closing.push(']');

    let rem = &input[open_len..];
    if let Some(pos) = rem.find(&closing) {
        let content = &rem[..pos];
        let consumed = open_len + pos + closing.len();
        return Some((content, consumed));
    }
    None
}