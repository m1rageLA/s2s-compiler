use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct IrDocField {
    pub name: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IrDocVariant {
    pub name: String,
    pub signature: String,
    #[serde(default)]
    pub fields: Vec<IrDocField>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IrDocNode {
    pub name: String,
    pub kind: String,
    pub signature: String,
    #[serde(default)]
    pub fields: Vec<IrDocField>,
    #[serde(default)]
    pub variants: Vec<IrDocVariant>,
}

const ITEM_SOURCE: &str = include_str!("item.rs");

pub fn ir_docs() -> Vec<IrDocNode> {
    parse_ir_file(ITEM_SOURCE)
}

fn parse_ir_file(source: &str) -> Vec<IrDocNode> {
    let mut nodes = Vec::new();
    let mut offset = 0;
    let source_bytes = source.as_bytes();

    while let Some((decl_start, kind)) = find_next_declaration(source, offset) {
        let name_start = decl_start + kind.len();
        let name_end = match source[name_start..].find(|c: char| c == '{' || c == '\n') {
            Some(pos) => name_start + pos,
            None => break,
        };
        let name = source[name_start..name_end]
            .trim()
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches(|c: char| c == '<' || c == '>')
            .to_string();

        let brace_pos = match source[name_end..].find('{') {
            Some(pos) => name_end + pos,
            None => break,
        };
        let (body, next_offset) = extract_braced_block(source, brace_pos);
        let signature = source[decl_start..next_offset].trim().to_string();

        if kind.starts_with("pub struct") {
            let fields = parse_struct_fields(&body);
            nodes.push(IrDocNode {
                name,
                kind: "struct".into(),
                signature,
                fields,
                variants: Vec::new(),
            });
        } else {
            let variants = parse_enum_variants(&body);
            nodes.push(IrDocNode {
                name,
                kind: "enum".into(),
                signature,
                fields: Vec::new(),
                variants,
            });
        }

        offset = next_offset;
        if offset >= source_bytes.len() {
            break;
        }
    }

    nodes
}

fn find_next_declaration(source: &str, offset: usize) -> Option<(usize, &'static str)> {
    let tail = &source[offset..];
    let struct_pos = tail.find("pub struct ").map(|p| (offset + p, "pub struct "));
    let enum_pos = tail.find("pub enum ").map(|p| (offset + p, "pub enum "));

    match (struct_pos, enum_pos) {
        (Some(s), Some(e)) => Some(if s.0 < e.0 { s } else { e }),
        (Some(s), None) => Some(s),
        (None, Some(e)) => Some(e),
        (None, None) => None,
    }
}

fn extract_braced_block(source: &str, open_brace_index: usize) -> (String, usize) {
    let mut depth = 0usize;
    let mut end_index = open_brace_index;
    for (idx, ch) in source[open_brace_index..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end_index = open_brace_index + idx + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    let body = source[open_brace_index + 1..end_index - 1].to_string();
    (body, end_index)
}

fn parse_struct_fields(body: &str) -> Vec<IrDocField> {
    split_top_level(body)
        .into_iter()
        .filter_map(|segment| {
            let segment = strip_comment(&segment);
            if segment.is_empty() {
                return None;
            }
            let seg = segment.trim().trim_end_matches(',');
            let mut parts = seg.splitn(2, ':');
            let name_part = parts.next()?.trim().trim_start_matches("pub ");
            let ty_part = parts.next()?.trim();
            Some(IrDocField {
                name: name_part.to_string(),
                signature: ty_part.to_string(),
            })
        })
        .collect()
}

fn parse_enum_variants(body: &str) -> Vec<IrDocVariant> {
    split_top_level(body)
        .into_iter()
        .filter_map(|segment| {
            let segment = strip_comment(&segment);
            if segment.is_empty() {
                return None;
            }
            let segment = segment.trim().trim_end_matches(',').to_string();
            let name = extract_variant_name(&segment)?;
            let fields = extract_variant_fields(&segment);
            Some(IrDocVariant {
                name,
                signature: segment,
                fields,
            })
        })
        .collect()
}

fn extract_variant_name(segment: &str) -> Option<String> {
    let mut name = String::new();
    for ch in segment.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            name.push(ch);
        } else {
            break;
        }
    }
    if name.is_empty() { None } else { Some(name) }
}

fn extract_variant_fields(segment: &str) -> Vec<IrDocField> {
    let remainder = segment
        .trim_start_matches(|c: char| c.is_alphanumeric() || c == '_')
        .trim();

    if remainder.is_empty() {
        return Vec::new();
    }

    if remainder.starts_with('{') {
        let inner = trim_enclosing(&remainder, '{', '}');
        parse_named_fields(&inner)
    } else if remainder.starts_with('(') {
        let inner = trim_enclosing(&remainder, '(', ')');
        parse_tuple_fields(&inner)
    } else {
        Vec::new()
    }
}

fn parse_named_fields(inner: &str) -> Vec<IrDocField> {
    split_top_level(inner)
        .into_iter()
        .enumerate()
        .filter_map(|(_idx, segment)| {
            let segment = strip_comment(&segment);
            if segment.is_empty() {
                return None;
            }
            let seg = segment.trim().trim_end_matches(',');
            let mut parts = seg.splitn(2, ':');
            let name = parts.next()?.trim();
            let ty = parts.next()?.trim();
            Some(IrDocField {
                name: name.to_string(),
                signature: ty.to_string(),
            })
        })
        .collect()
}

fn parse_tuple_fields(inner: &str) -> Vec<IrDocField> {
    split_top_level(inner)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, segment)| {
            let segment = strip_comment(&segment);
            if segment.is_empty() {
                return None;
            }
            let sig = segment.trim().trim_end_matches(',');
            Some(IrDocField {
                name: idx.to_string(),
                signature: sig.to_string(),
            })
        })
        .collect()
}

fn strip_comment(segment: &str) -> String {
    match segment.find("//") {
        Some(pos) => segment[..pos].trim().to_string(),
        None => segment.trim().to_string(),
    }
}

fn trim_enclosing(text: &str, open: char, close: char) -> String {
    let mut chars = text.chars();
    if chars.next() != Some(open) {
        return text.to_string();
    }
    let mut buf = String::new();
    let mut depth = 0usize;
    for ch in chars {
        if ch == open {
            depth += 1;
        } else if ch == close {
            if depth == 0 {
                break;
            }
            depth -= 1;
        }
        buf.push(ch);
    }
    buf
}

fn split_top_level(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    let chars: Vec<char> = text.chars().collect();
    for (idx, ch) in chars.iter().enumerate() {
        match ch {
            '{' | '(' | '[' => depth += 1,
            '}' | ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                let slice = text[start..idx].to_string();
                if !slice.trim().is_empty() {
                    parts.push(slice);
                }
                start = idx + 1;
            }
            _ => {}
        }
    }
    if start < text.len() {
        let slice = text[start..].to_string();
        if !slice.trim().is_empty() {
            parts.push(slice);
        }
    }
    parts
}
