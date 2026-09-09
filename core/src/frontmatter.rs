//! Splits a Pussla note (YAML frontmatter + Markdown body) into its two parts,
//! and parses the frontmatter into a typed model.
//!
//! Format (see ADR-002): a `---`-delimited YAML block, then a Markdown body.

use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrontmatterError {
    #[error("file has no frontmatter block (expected leading `---`)")]
    MissingOpeningDelimiter,
    #[error("frontmatter block is not closed (expected a second `---`)")]
    MissingClosingDelimiter,
    #[error("failed to parse frontmatter YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

/// A note split into its raw frontmatter YAML text and Markdown body.
pub struct RawNote<'a> {
    pub frontmatter_yaml: &'a str,
    pub body: &'a str,
}

/// Split raw note text into (frontmatter YAML, body), without parsing the YAML.
pub fn split(raw: &str) -> Result<RawNote<'_>, FrontmatterError> {
    let mut lines = raw.splitn(2, '\n');
    let first_line = lines.next().unwrap_or("");
    if first_line.trim_end() != "---" {
        return Err(FrontmatterError::MissingOpeningDelimiter);
    }
    let rest = lines.next().unwrap_or("");

    let delimiter_pos = find_closing_delimiter(rest)
        .ok_or(FrontmatterError::MissingClosingDelimiter)?;

    let frontmatter_yaml = &rest[..delimiter_pos.yaml_end];
    let body = rest[delimiter_pos.body_start..].trim_start_matches('\n');

    Ok(RawNote {
        frontmatter_yaml,
        body,
    })
}

struct DelimiterPos {
    yaml_end: usize,
    body_start: usize,
}

fn find_closing_delimiter(rest: &str) -> Option<DelimiterPos> {
    let mut offset = 0usize;
    for line in rest.split('\n') {
        let line_with_newline_len = line.len() + 1; // account for the '\n' split() consumed
        if line.trim_end() == "---" {
            return Some(DelimiterPos {
                yaml_end: offset,
                body_start: offset + line_with_newline_len,
            });
        }
        offset += line_with_newline_len;
    }
    None
}

/// Parse a full note's frontmatter into `T`, discarding the body.
pub fn parse_frontmatter<T: DeserializeOwned>(raw: &str) -> Result<T, FrontmatterError> {
    let note = split(raw)?;
    Ok(serde_yaml::from_str(note.frontmatter_yaml)?)
}

/// Parse a full note into (frontmatter as `T`, body as owned String).
pub fn parse<T: DeserializeOwned>(raw: &str) -> Result<(T, String), FrontmatterError> {
    let note = split(raw)?;
    let fm = serde_yaml::from_str(note.frontmatter_yaml)?;
    Ok((fm, note.body.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Simple {
        alias: String,
    }

    #[test]
    fn splits_frontmatter_and_body() {
        let raw = "---\nalias: FishCatcher\n---\nHello body.\n";
        let note = split(raw).unwrap();
        assert_eq!(note.frontmatter_yaml.trim(), "alias: FishCatcher");
        assert_eq!(note.body, "Hello body.\n");
    }

    #[test]
    fn parses_typed_frontmatter() {
        let raw = "---\nalias: FishCatcher\n---\nBody.\n";
        let (fm, body): (Simple, String) = parse(raw).unwrap();
        assert_eq!(fm.alias, "FishCatcher");
        assert_eq!(body, "Body.\n");
    }

    #[test]
    fn missing_opening_delimiter_errors() {
        let raw = "alias: FishCatcher\n---\nBody\n";
        assert!(matches!(
            split(raw),
            Err(FrontmatterError::MissingOpeningDelimiter)
        ));
    }

    #[test]
    fn missing_closing_delimiter_errors() {
        let raw = "---\nalias: FishCatcher\nBody\n";
        assert!(matches!(
            split(raw),
            Err(FrontmatterError::MissingClosingDelimiter)
        ));
    }

    #[test]
    fn empty_body_after_frontmatter() {
        let raw = "---\nalias: FishCatcher\n---\n";
        let note = split(raw).unwrap();
        assert_eq!(note.body, "");
    }
}
