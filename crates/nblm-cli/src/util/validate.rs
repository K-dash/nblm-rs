use anyhow::{anyhow, bail, Result};

pub fn validate_url(url: &str) -> Result<()> {
    let parsed = url::Url::parse(url).map_err(|err| anyhow!("invalid URL {url}: {err}"))?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        other => bail!("unsupported URL scheme: {other}"),
    }
}

pub fn pair_with_names(
    values: &[String],
    names: &[String],
    field: &str,
) -> Result<Vec<(String, Option<String>)>> {
    if names.len() > values.len() {
        bail!("{field} count exceeds number of values");
    }
    Ok(values
        .iter()
        .enumerate()
        .map(|(idx, value)| {
            let name = names.get(idx).and_then(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            });
            (value.clone(), name)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_pairs_names() {
        let values = vec!["https://example.com".to_string()];
        let names = vec!["  Example  ".to_string()];
        let pairs = pair_with_names(&values, &names, "--name").unwrap();
        assert_eq!(pairs[0].0, "https://example.com");
        assert_eq!(pairs[0].1.as_deref(), Some("Example"));
    }

    #[test]
    fn empty_name_becomes_none() {
        let values = vec!["https://example.com".to_string()];
        let names = vec!["   ".to_string()];
        let pairs = pair_with_names(&values, &names, "--name").unwrap();
        assert!(pairs[0].1.is_none());
    }

    #[test]
    fn len_mismatch_errors() {
        let values = vec!["https://example.com".to_string()];
        let names = vec!["one".to_string(), "two".to_string()];
        assert!(pair_with_names(&values, &names, "--name").is_err());
    }
}
