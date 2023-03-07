use reqwest::header::HeaderMap;
use std::borrow::Cow;

pub(crate) fn headers_repr(map: &HeaderMap) -> String {
    map.iter()
        .map(|(k, v)| {
            format!(
                "{k}: {}",
                if v.is_sensitive() {
                    "<sensitive>"
                } else {
                    v.to_str().unwrap_or("<n/a>")
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_utf8_bom(b: &[u8]) -> &[u8] {
    match b {
        &[0xEF, 0xBB, 0xBF, ..] => &b[3..],
        _ => b,
    }
}

fn take_only_n_bytes(b: &[u8], count: usize) -> &[u8] {
    if b.len() > count {
        &b[0..count]
    } else {
        b
    }
}

/// Attempt to convert a buffer to a string representation for tracing.
pub fn text_repr(bytes: &[u8]) -> Cow<str> {
    const KB: usize = 1024;

    String::from_utf8_lossy(take_only_n_bytes(strip_utf8_bom(bytes), 30 * KB))
}
